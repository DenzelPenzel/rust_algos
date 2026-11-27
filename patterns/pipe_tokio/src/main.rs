use std::{future::Future, sync::Arc, collections::VecDeque};
use tokio::{
    sync::{Mutex, mpsc, oneshot},
    task::JoinSet,
};

const MAX_ITEMS: usize = 9_999;
type Cookie = i64;

trait Source: Send + 'static {
    type Item: Send + Sync + 'static;
    type Error: Send + 'static;

    fn next(
        &mut self,
    ) -> impl Future<Output = Result<(Vec<Self::Item>, Cookie), Self::Error>> + Send;
}

trait Committer: Send + 'static {
    type Error: Send + 'static;

    fn commit(&mut self, cookie: Cookie) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

pub trait Consumer: Send + 'static {
    type Item: Send + Sync + 'static;
    type Error: Send + 'static;

    fn process(
        &mut self,
        items: &[Self::Item],
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

#[derive(Debug, Clone, Copy)]
struct PipeConfig {
    queue_capacity: usize,
}

impl Default for PipeConfig {
    fn default() -> Self {
        Self { queue_capacity: 8 }
    }
}

#[derive(Debug, PartialEq)]
pub enum PipeError<SourceError, ConsumerError, CommitError> {
    Source(SourceError),

    Consumer { sequence: u64, error: ConsumerError },

    Commit(CommitError),

    SourceBatchTooLarge { actual: usize, max: usize },

    NoConsumers,
    InvalidQueueCapacity,
    PipelineStopped,
    TaskPanicked,
}

struct Job<T> {
    seq: u64,
    items: Vec<T>,
    cookies: Vec<Cookie>,
    completed: oneshot::Sender<Vec<Cookie>>,
}

type Ticker = oneshot::Receiver<Vec<Cookie>>;

enum PipelineFailure<SourceError, ConsumerError> {
    Source(SourceError),
    Consumer { sequence: u64, error: ConsumerError },
    SourceBatchTooLarge(usize),
    PipelineStopped,
}

async fn send_job<T>(
    jobs: &mpsc::Sender<Job<T>>,
    ordered: &mpsc::Sender<Option<Ticker>>,
    seq: &mut u64,
    items: &mut Vec<T>,
    cookies: &mut Vec<Cookie>,
) -> Result<(), ()> {
    let (competed_tx, completed_rx) = oneshot::channel();
    let old_items = std::mem::replace(items, Vec::with_capacity(MAX_ITEMS));
    let job = Job {
        seq: *seq,
        items: old_items,
        cookies: std::mem::take(cookies),
        completed: competed_tx,
    };

    jobs.send(job).await.map_err(|_| ())?;
    ordered.send(Some(completed_rx)).await.map_err(|_| ())?;

    *seq += 1;
    Ok(())
}

async fn run_worker<C, SourceError>(
    mut consumer: C,
    jobs: Arc<Mutex<mpsc::Receiver<Job<C::Item>>>>,
) -> Result<(), PipelineFailure<SourceError, C::Error>>
where
    C: Consumer,
    SourceError: Send + 'static,
{
    loop {
        let job = {
            let mut jobs = jobs.lock().await;
            jobs.recv().await
        };

        let Some(job) = job else {
            return Ok(());
        };

        if !job.items.is_empty() {
            consumer
                .process(&job.items)
                .await
                .map_err(|error| PipelineFailure::Consumer {
                    sequence: job.seq,
                    error,
                })?;
        }

        job.completed
            .send(job.cookies)
            .map_err(|_| PipelineFailure::PipelineStopped)?;
    }
}

fn into_pipe_error<SourceError, ConsumerError, CommitError>(
    failure: PipelineFailure<SourceError, ConsumerError>,
) -> PipeError<SourceError, ConsumerError, CommitError> {
    match failure {
        PipelineFailure::Source(error) => PipeError::Source(error),
        PipelineFailure::Consumer { sequence, error } => PipeError::Consumer { sequence, error },
        PipelineFailure::SourceBatchTooLarge(actual) => PipeError::SourceBatchTooLarge {
            actual,
            max: MAX_ITEMS,
        },
        PipelineFailure::PipelineStopped => PipeError::PipelineStopped,
    }
}

async fn wait_or_fail<F, SourceError, ConsumerError, CommitError>(
    future: F,
    failures: &mut mpsc::UnboundedReceiver<PipelineFailure<SourceError, ConsumerError>>,
    tasks: &mut JoinSet<()>,
) -> Result<F::Output, PipeError<SourceError, ConsumerError, CommitError>>
where
    F: Future,
{
    tokio::pin!(future);
    let mut failures_open = true;

    loop {
        tokio::select! {
            biased;

            failure = failures.recv(), if failures_open => {
                match failure {
                    Some(failure) => return Err(into_pipe_error(failure)),
                    None => failures_open = false,
                }
            }

            task = tasks.join_next(), if !tasks.is_empty() => {
                if matches!(task, Some(Err(_))) {
                    return Err(PipeError::TaskPanicked);
                }
            }

            result = &mut future => return Ok(result),
        }
    }
}

async fn pipe_parallel<S, K, C>(
    source: S,
    mut committer: K,
    consumers: Vec<C>,
    config: PipeConfig,
) -> Result<(), PipeError<S::Error, C::Error, K::Error>>
where
    S: Source,
    K: Committer,
    C: Consumer<Item = S::Item>,
{
    let (jobs_tx, jobs_rx) = mpsc::channel(config.queue_capacity);

    let jobs_rx = Arc::new(Mutex::new(jobs_rx));
    let (ordered_tx, mut ordered_rx) = mpsc::channel(config.queue_capacity);
    let (failures_tx, mut failures_rx) = mpsc::unbounded_channel();

    let mut tasks = JoinSet::new();

    let batcher_failures = failures_tx.clone();

    tasks.spawn(async move {
        let result = async {
            let mut source = source;
            let mut seq = 0;
            let mut items = Vec::with_capacity(MAX_ITEMS);
            let mut cookies = Vec::new();

            loop {
                let (mut next_items, cookie) = source
                    .next()
                    .await
                    .map_err(PipelineFailure::<S::Error, C::Error>::Source)?;

                if next_items.len() > MAX_ITEMS {
                    return Err(PipelineFailure::SourceBatchTooLarge(next_items.len()));
                }

                let end_of_stream = next_items.is_empty();
                let next_batch_does_not_fit =
                    !items.is_empty() && items.len() + next_items.len() > MAX_ITEMS;

                if next_batch_does_not_fit {
                    send_job(&jobs_tx, &ordered_tx, &mut seq, &mut items, &mut cookies)
                        .await
                        .map_err(|_| PipelineFailure::PipelineStopped)?;
                }

                items.append(&mut next_items);
                cookies.push(cookie);

                if items.len() == MAX_ITEMS || end_of_stream {
                    send_job(&jobs_tx, &ordered_tx, &mut seq, &mut items, &mut cookies)
                        .await
                        .map_err(|_| PipelineFailure::PipelineStopped)?;
                }

                if end_of_stream {
                    ordered_tx
                        .send(None)
                        .await
                        .map_err(|_| PipelineFailure::PipelineStopped)?;
                    return Ok(());
                }
            }
        }
        .await;

        if let Err(error) = result {
            let _ = batcher_failures.send(error);
        }
    });

    for consumer in consumers {
        let worker_jobs = Arc::clone(&jobs_rx);
        let worker_failures = failures_tx.clone();

        tasks.spawn(async move {
            if let Err(error) = run_worker::<C, S::Error>(consumer, worker_jobs).await {
                let _ = worker_failures.send(error);
            }
        });
    }

    drop(jobs_rx);
    drop(failures_tx);

    let mut res = async {
        loop {
            let Some(ticket) = wait_or_fail::<_, S::Error, C::Error, K::Error>(
                ordered_rx.recv(),
                &mut failures_rx,
                &mut tasks,
            )
            .await?
            .ok_or(PipeError::PipelineStopped)?
            else {
                return Ok(());
            };

            let cookies = wait_or_fail::<_, S::Error, C::Error, K::Error>(
                ticket,
                &mut failures_rx,
                &mut tasks,
            )
            .await?
            .map_err(|_| PipeError::PipelineStopped)?;

            for cookie in cookies {
                committer.commit(cookie).await.map_err(PipeError::Commit)?;
            }
        }
    }
    .await;

    if res.is_err() {
        tasks.abort_all();
    }

    while let Some(task_res) = tasks.join_next().await {
        if task_res.is_err() && res.is_ok() {
            res = Err(PipeError::TaskPanicked);
        }
    }

    res
}

#[tokio::main]
async fn main() {
    // 15_000 items
    // Cookie 1..300 per 50 items
    let mut batches = VecDeque::new();

    for cookie in 1..=300 {
        batches.push_back((
            vec![cookie as u32; 50],
            cookie,
        ));
    }

    batches.push_back((Vec::new(), 301));

    let source = DemoSource { batches };
    let committer = DemoCommitter;
    let consumers = vec![
        DemoConsumer { id: 1 },
        DemoConsumer { id: 2 },
        DemoConsumer { id: 3 },
    ];

    let result = pipe_parallel(
            source,
            committer,
            consumers,
            PipeConfig {
                queue_capacity: 4,
            },
        )
        .await;


    match result {
        Ok(()) => println!("pipe completed"),
        Err(error) => eprintln!("pipe failed: {error:?}"),
    }
}


struct DemoSource {
    batches: VecDeque<(Vec<u32>, Cookie)>,
}

impl Source for DemoSource {
    type Item = u32;
    type Error = &'static str;

    async fn next(
        &mut self,
    ) -> Result<(Vec<Self::Item>, Cookie), Self::Error> {
        self.batches
            .pop_front()
            .ok_or("source exhausted")
    }
}

struct DemoCommitter;

impl Committer for DemoCommitter {
    type Error = &'static str;

    async fn commit(
        &mut self,
        cookie: Cookie,
    ) -> Result<(), Self::Error> {
        println!("commit cookie={cookie}");
        Ok(())
    }
}

struct DemoConsumer {
    id: usize,
}

impl Consumer for DemoConsumer {
    type Item = u32;
    type Error = &'static str;

    async fn process(
        &mut self,
        items: &[Self::Item],
    ) -> Result<(), Self::Error> {
        println!(
            "worker={} process {} items",
            self.id,
            items.len(),
        );

        tokio::task::yield_now().await;

        Ok(())
    }
}
