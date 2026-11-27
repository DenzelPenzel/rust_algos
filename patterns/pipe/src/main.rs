/*
Нам нужно передать данные из некоторого источника некоторому потребителю.
При этом источник отдает данные небольшими пачками (~ десятки записей), а потребитель оптимальнее работает с крупными батчами (~тысячи записей).
Реальный пример - поставка данных из очередей типа Kafka в базу Clickhouse.

Источник:
- Условно бесконечный.
- Источник никогда не возвращает более MaxItems записей за один вызов Next.
- В рамках одной "сессии" (одного вызова функции Pipe) источник каждый раз возвращает новые данные на каждый вызов Next.
- Однако, после перезапуска источник начнет с прошлой "подтвержденной" позиции, задаваемой cookie.
    Поэтому *каждое* значение cookie, которое вернул вызов Next, после сохранения данных в приемнике,
    должно быть фиксировано вызовом Commit, причем строго в той же последовательности, в которой их вернул Next

Приемник:
- Не может обработать более MaxItems за один раз.
*/

use std::{collections::VecDeque, sync::mpsc, thread};

pub const MAX_ITEMS: usize = 9999;

pub type Cookie = i64;

pub trait Producer {
    type Item;
    type Error;

    fn next(&mut self) -> Result<(Vec<Self::Item>, Cookie), Self::Error>;
    fn commit(&mut self, cookie: Cookie) -> Result<(), Self::Error>;
}

pub trait Consumer {
    type Item;
    type Error;

    fn process(&mut self, items: &[Self::Item]) -> Result<(), Self::Error>;
}

#[derive(Debug, PartialEq)]
pub enum PipeError<ProducerError, ConsumerError> {
    Producer(ProducerError),
    Consumer(ConsumerError),
    WorkerStopped,
}

fn flush<P, C>(
    producer: &mut P,
    consumer: &mut C,
    items: &mut Vec<P::Item>,
    cookies: &mut Vec<Cookie>,
) -> Result<(), PipeError<P::Error, C::Error>>
where
    P: Producer,
    C: Consumer<Item = P::Item>,
{
    if items.is_empty() {
        return Ok(());
    }

    consumer.process(items).map_err(PipeError::Consumer)?;
    for cookie in cookies.drain(..) {
        producer.commit(cookie).map_err(PipeError::Producer)?;
    }

    items.clear();
    Ok(())
}

// НУЖНО добавить параллелизм для этой фукнции процессинга process и commit
fn pipe<P, C>(producer: &mut P, consumer: &mut C) -> Result<(), PipeError<P::Error, C::Error>>
where
    P: Producer,
    C: Consumer<Item = P::Item>,
{
    let mut batch = Vec::new();
    let mut cookies = Vec::new();

    loop {
        let (mut items, cookie) = producer.next().map_err(PipeError::Producer)?;
        let end_of_stream = items.is_empty();

        if batch.len() + items.len() > MAX_ITEMS {
            flush(producer, consumer, &mut batch, &mut cookies)?;
        }

        batch.append(&mut items);
        cookies.push(cookie);

        if batch.len() == MAX_ITEMS || end_of_stream {
            flush(producer, consumer, &mut batch, &mut cookies)?;
        }

        if end_of_stream {
            return Ok(());
        }
    }
}

fn pipe2<P, C>(producer: &mut P, consumer: &mut C) -> Result<(), PipeError<P::Error, C::Error>>
where
    P: Producer,
    P::Item: Send,
    C: Consumer<Item = P::Item> + Send,
    C::Error: Send,
{
    let mut items = Vec::new();
    let mut cookeis = Vec::new();
    let mut prefetched = VecDeque::new();
    let mut end_of_stream = false;

    loop {
        if end_of_stream && prefetched.is_empty() {
            return flush(producer, consumer, &mut items, &mut cookeis);
        }

        let (mut next_items, cookie) = if let Some(batch) = prefetched.pop_front() {
            batch
        } else {
            producer.next().map_err(PipeError::Producer)?
        };

        if next_items.is_empty() {
            end_of_stream = true;
            continue;
        }

        if next_items.len() <= MAX_ITEMS - items.len() {
            items.append(&mut next_items);
            cookeis.push(cookie);
            if items.len() < MAX_ITEMS {
                continue;
            }
        } else {
            prefetched.push_front((next_items, cookie));
        }

        let batch = std::mem::take(&mut items);
        let batch_cookies = std::mem::take(&mut cookeis);

        let consumer_ref = &mut *consumer;

        let (precess_res, next_res) = thread::scope(|s| {
            let worker = s.spawn(move || consumer_ref.process(&batch));
            let next_res = if end_of_stream {
                None
            } else {
                Some(producer.next())
            };
            let process_res = match worker.join() {
                Ok(res) => res,
                Err(panic) => std::panic::resume_unwind(panic),
            };
            (process_res, next_res)
        });

        precess_res.map_err(PipeError::Consumer)?;

        for cookie in batch_cookies {
            producer.commit(cookie).map_err(PipeError::Producer)?;
        }

        if let Some(res) = next_res {
            let batch = res.map_err(PipeError::Producer)?;
            if batch.0.is_empty() {
                end_of_stream = true;
            } else {
                prefetched.push_back(batch);
            }
        }
    }
}

struct Job<T> {
    seq: u64,
    items: Vec<T>,
    cookies: Vec<Cookie>,
}

impl<T> Job<T> {
    fn with_capacity() -> Self {
        Self {
            seq: 0,
            items: Vec::with_capacity(MAX_ITEMS),
            cookies: Vec::new(),
        }
    }
}

fn collect_batch<P>(
    producer: &mut P,
    pending: &mut VecDeque<(Vec<P::Item>, Cookie)>,
    mut job: Job<P::Item>,
) -> Result<(Job<P::Item>, bool), P::Error>
where
    P: Producer,
{
    job.items.clear();
    job.cookies.clear();

    loop {
        let (mut next_items, cookie) = match pending.pop_front() {
            Some(batch) => batch,
            None => producer.next()?,
        };

        if next_items.is_empty() {
            job.cookies.push(cookie);
            return Ok((job, true));
        }

        let rem = MAX_ITEMS - job.items.len();

        if next_items.len() > rem {
            pending.push_front((next_items, cookie));
            return Ok((job, false));
        }

        job.items.append(&mut next_items);
        job.cookies.push(cookie);

        if job.items.len() == MAX_ITEMS {
            return Ok((job, false));
        }
    }
}

fn commit_job<P, ConsumerError>(
    producer: &mut P,
    job: &mut Job<P::Item>,
) -> Result<(), PipeError<P::Error, ConsumerError>>
where
    P: Producer,
{
    for cookie in job.cookies.drain(..) {
        producer.commit(cookie).map_err(PipeError::Producer)?;
    }
    job.items.clear();
    Ok(())
}

fn pipe3<P, C>(producer: &mut P, consumer: &mut C) -> Result<(), PipeError<P::Error, C::Error>>
where
    P: Producer,
    P::Item: Send,
    C: Consumer<Item = P::Item> + Send,
    C::Error: Send,
{
    thread::scope(|score| {
        let (job_tx, job_rx) = mpsc::sync_channel::<Job<P::Item>>(1);

        let (done_tx, done_rx) = mpsc::channel::<(Job<P::Item>, Result<(), C::Error>)>();

        score.spawn(move || {
            while let Ok(job) = job_rx.recv() {
                let process_result = if job.items.is_empty() {
                    Ok(())
                } else {
                    consumer.process(&job.items)
                };

                if done_tx.send((job, process_result)).is_err() {
                    break;
                }
            }
        });

        let pipeline_result = (|| {
            let mut pending = VecDeque::new();

            let first_buffer = Job::with_capacity();

            let (first_job, mut current_is_last) =
                collect_batch(producer, &mut pending, first_buffer).map_err(PipeError::Producer)?;

            job_tx
                .send(first_job)
                .map_err(|_| PipeError::WorkerStopped)?;

            let mut spare = Job::with_capacity();

            loop {
                let next = if current_is_last {
                    None
                } else {
                    Some(
                        collect_batch(producer, &mut pending, spare)
                            .map_err(PipeError::Producer)?,
                    )
                };

                let (mut processed, process_result) =
                    done_rx.recv().map_err(|_| PipeError::WorkerStopped)?;

                process_result.map_err(PipeError::Consumer)?;

                match next {
                    Some((next_job, next_is_last)) => {
                        job_tx
                            .send(next_job)
                            .map_err(|_| PipeError::WorkerStopped)?;

                        commit_job::<P, C::Error>(producer, &mut processed)?;

                        spare = processed;
                        current_is_last = next_is_last;
                    }
                    None => {
                        commit_job::<P, C::Error>(producer, &mut processed)?;
                        return Ok(());
                    }
                }
            }
        })();

        drop(job_tx);
        drop(done_rx);

        pipeline_result
    })
}

fn main() {
    println!("Hello, world!");
}
