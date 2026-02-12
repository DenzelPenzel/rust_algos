use futures::{
    channel::oneshot,
    prelude::*,
    stream::FuturesUnordered,
};
use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    pin::Pin,
    sync::{Arc, Mutex},
};
use tokio::time::{interval, Duration};
use tokio_stream::wrappers::IntervalStream;

type PeerId = String;

type TaskId = u64;

#[derive(Debug, Clone)]
struct Task {
    id: TaskId,
    data: String,
}

#[derive(Debug, Clone, PartialEq)]
enum TaskResult {
    Success(String),
    AlreadyProcessed,
    Invalid(String),
}

mod reputation {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ReputationChange {
        pub value: i32,
        pub reason: &'static str,
    }

    pub const TASK_RECEIVED: ReputationChange = ReputationChange {
        value: -10,
        reason: "Task received (pending validation)",
    };
    pub const TASK_VALID: ReputationChange = ReputationChange {
        value: 100,
        reason: "Valid task",
    };
    pub const TASK_REFUND: ReputationChange = ReputationChange {
        value: 10,
        reason: "Task validation refund",
    };
    pub const DUPLICATE_TASK: ReputationChange = ReputationChange {
        value: -50,
        reason: "Duplicate task from same peer",
    };
    pub const INVALID_TASK: ReputationChange = ReputationChange {
        value: -200,
        reason: "Invalid task",
    };
}

trait TaskStore: Send + Sync {
    fn has_task(&self, id: TaskId) -> bool;
    fn submit(&self, task: Task) -> TaskResult;
}

#[derive(Clone)]
struct InMemoryTaskStore {
    tasks: Arc<Mutex<HashMap<TaskId, Task>>>,
}

impl InMemoryTaskStore {
    fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl TaskStore for InMemoryTaskStore {
    fn has_task(&self, id: TaskId) -> bool {
        self.tasks.lock().unwrap().contains_key(&id)
    }

    fn submit(&self, task: Task) -> TaskResult {
        let mut tasks = self.tasks.lock().unwrap();

        if task.data.contains("invalid") {
            return TaskResult::Invalid("Task contains invalid data".to_string());
        }

        match tasks.entry(task.id) {
            Entry::Vacant(e) => {
                e.insert(task.clone());
                TaskResult::Success(format!("Processed: {}", task.data))
            }
            Entry::Occupied(_) => TaskResult::AlreadyProcessed,
        }
    }
}

trait Network: Send + Sync {
    fn report_peer(&self, peer: PeerId, change: reputation::ReputationChange);
}

#[derive(Clone)]
struct MockNetwork {
    reports: Arc<Mutex<Vec<(PeerId, reputation::ReputationChange)>>>,
}

impl MockNetwork {
    fn new() -> Self {
        Self {
            reports: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn get_reports(&self) -> Vec<(PeerId, reputation::ReputationChange)> {
        self.reports.lock().unwrap().clone()
    }
}

impl Network for MockNetwork {
    fn report_peer(&self, peer: PeerId, change: reputation::ReputationChange) {
        println!(
            "📊 Reputation: {} → {:+} ({})",
            peer, change.value, change.reason
        );
        self.reports.lock().unwrap().push((peer, change));
    }
}

const MAX_PENDING_TASKS: usize = 100;
const PROCESS_INTERVAL: Duration = Duration::from_millis(100);

struct TaskHandler<N: Network> {
    network: N,
    task_store: Arc<dyn TaskStore>,

    /// Channel to send tasks to workers
    queue_sender: async_channel::Sender<(Task, oneshot::Sender<TaskResult>)>,

    /// Pending task futures waiting for worker completion
    pending_tasks:
        FuturesUnordered<Pin<Box<dyn Future<Output = (TaskId, Option<TaskResult>)> + Send>>>,

    /// Track which peers sent which pending tasks
    pending_tasks_peers: HashMap<TaskId, HashSet<PeerId>>,

    process_timer: Pin<Box<dyn Stream<Item = tokio::time::Instant> + Send>>,
}

impl<N: Network> TaskHandler<N> {
    fn new(network: N, task_store: Arc<dyn TaskStore>, num_workers: usize) -> Self {
        let (queue_sender, queue_receiver) = async_channel::bounded(MAX_PENDING_TASKS);

        for worker_id in 0..num_workers {
            let receiver: async_channel::Receiver<(Task, oneshot::Sender<TaskResult>)> = queue_receiver.clone();
            let store = task_store.clone();

            tokio::spawn(async move {
                println!("🔧 Worker {} started", worker_id);
                loop {
                    match receiver.recv().await {
                        Err(_) => {
                            println!("🔧 Worker {} shutting down", worker_id);
                            return;
                        }
                        Ok((task, completion)) => {
                            println!("🔧 Worker {} processing task {}", worker_id, task.id);

                            // Simulate some processing time
                            tokio::time::sleep(Duration::from_millis(10)).await;

                            let res = store.submit(task);

                            if completion.send(res).is_err() {
                                println!("⚠️  Worker {} failed to send completion", worker_id);
                            }
                        }
                    }
                }
            });
        }

        Self {
            network,
            task_store,
            queue_sender,
            pending_tasks: FuturesUnordered::new(),
            pending_tasks_peers: HashMap::new(),
            process_timer: Box::pin(IntervalStream::new(interval(PROCESS_INTERVAL))),
        }
    }

    /// Handle incoming tasks from a peer (main entry point)
    fn on_tasks(&mut self, peer: PeerId, tasks: Vec<Task>) {
        println!("📥 Received {} tasks from {}", tasks.len(), peer);

        for task in tasks {
            let task_id = task.id;

            // Check if already in store
            if self.task_store.has_task(task_id) {
                println!("⏭️  Task {} already in store", task_id);
                println!("⚠️  Duplicate task {} from peer {}", task_id, peer);
                self.network.report_peer(peer.clone(), reputation::DUPLICATE_TASK);
                continue;
            }

            // Try to send to worker queue
            match self.pending_tasks_peers.entry(task_id) {
                Entry::Vacant(entry) => {
                    // New task - report receipt
                    self.network.report_peer(peer.clone(), reputation::TASK_RECEIVED);

                    let (completion_sender, completion_receiver) = oneshot::channel();

                    match self.queue_sender.try_send((task, completion_sender)) {
                        Ok(()) => {
                            println!("✅ Task {} queued for validation", task_id);

                            // Create future to wait for worker result
                            self.pending_tasks.push(
                                async move {
                                    let res = completion_receiver.await;
                                    (task_id, res.ok())
                                }
                                .boxed(),
                            );

                            // Track which peer sent this task
                            entry.insert(HashSet::from_iter([peer.clone()]));
                        }
                        Err(async_channel::TrySendError::Full(_)) => {
                            println!("⚠️  Queue full, dropping task {}", task_id);
                        }
                        Err(async_channel::TrySendError::Closed(_)) => {
                            println!("⚠️  Queue closed, dropping task {}", task_id);
                        }
                    }
                }
                Entry::Occupied(mut entry) => {
                    // Task is already pending
                    if !entry.get_mut().insert(peer.clone()) {
                        // Same peer sent it twice while pending - this is a duplicate
                        println!(
                            "⚠️  Duplicate task {} from same peer {} (while pending)",
                            task_id, peer
                        );
                        self.network.report_peer(peer.clone(), reputation::DUPLICATE_TASK);
                    } else {
                        // Different peer sent same task - report receipt
                        println!("ℹ️  Task {} also received from {}", task_id, peer);
                        self.network.report_peer(peer.clone(), reputation::TASK_RECEIVED);
                    }
                }
            }
        }
    }

    /// Handle completion of a task validation
    fn on_task_completed(&mut self, task_id: TaskId, res: TaskResult) {
        if let Some(peers) = self.pending_tasks_peers.remove(&task_id) {
            println!("✅ Task {} completed: {:?}", task_id, res);

            for peer in peers {
                match &res {
                    TaskResult::Success(_) => {
                        self.network.report_peer(peer, reputation::TASK_VALID);
                    }
                    TaskResult::AlreadyProcessed => {
                        self.network.report_peer(peer, reputation::TASK_REFUND);
                    }
                    TaskResult::Invalid(_) => {
                        self.network.report_peer(peer, reputation::INVALID_TASK);
                    }
                }
            }
        }
    }

    async fn run(mut self) {
        println!("🚀 Handler started");

        loop {
            tokio::select! {
                Some((task_id, result)) = self.pending_tasks.next() => {
                    if let Some(result) = result {
                        self.on_task_completed(task_id, result);
                    }
                }
                _ = self.process_timer.next() => {
                    println!("⏰ Periodic tick - {} pending tasks", self.pending_tasks.len());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_task_processing() {
        let network = MockNetwork::new();
        let store = Arc::new(InMemoryTaskStore::new());
        let mut handler = TaskHandler::new(network.clone(), store.clone(), 2);

        let task = Task {
            id: 1,
            data: "test task".to_string(),
        };

        handler.on_tasks("peer1".to_string(), vec![task]);

        // Wait for processing
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Process one completion
        if let Some((task_id, result)) = handler.pending_tasks.next().await {
            handler.on_task_completed(task_id, result.unwrap());
        }

        let reports = network.get_reports();
        assert_eq!(reports.len(), 2);
        assert_eq!(reports[0], ("peer1".to_string(), reputation::TASK_RECEIVED));
        assert_eq!(reports[1], ("peer1".to_string(), reputation::TASK_VALID));
    }

    #[tokio::test]
    async fn test_duplicate_detection() {
        let network = MockNetwork::new();
        let store = Arc::new(InMemoryTaskStore::new());
        let mut handler = TaskHandler::new(network.clone(), store.clone(), 2);

        let task = Task {
            id: 1,
            data: "test task".to_string(),
        };

        // First submission
        handler.on_tasks("peer1".to_string(), vec![task.clone()]);

        // Wait for processing
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Process completion
        if let Some((task_id, result)) = handler.pending_tasks.next().await {
            handler.on_task_completed(task_id, result.unwrap());
        }

        // Second submission (duplicate)
        handler.on_tasks("peer1".to_string(), vec![task]);

        let reports = network.get_reports();
        assert_eq!(reports.len(), 3);
        assert_eq!(reports[0], ("peer1".to_string(), reputation::TASK_RECEIVED));
        assert_eq!(reports[1], ("peer1".to_string(), reputation::TASK_VALID));
        assert_eq!(
            reports[2],
            ("peer1".to_string(), reputation::DUPLICATE_TASK)
        );
    }

    #[tokio::test]
    async fn test_invalid_task() {
        let network = MockNetwork::new();
        let store = Arc::new(InMemoryTaskStore::new());
        let mut handler = TaskHandler::new(network.clone(), store.clone(), 2);

        let task = Task {
            id: 1,
            data: "invalid task".to_string(),
        };

        handler.on_tasks("peer1".to_string(), vec![task]);

        // Wait for processing
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Process completion
        if let Some((task_id, result)) = handler.pending_tasks.next().await {
            handler.on_task_completed(task_id, result.unwrap());
        }

        let reports = network.get_reports();
        assert_eq!(reports.len(), 2);
        assert_eq!(reports[0], ("peer1".to_string(), reputation::TASK_RECEIVED));
        assert_eq!(reports[1], ("peer1".to_string(), reputation::INVALID_TASK));
    }

    #[tokio::test]
    async fn test_multiple_peers_same_task() {
        let network = MockNetwork::new();
        let store = Arc::new(InMemoryTaskStore::new());
        let mut handler = TaskHandler::new(network.clone(), store.clone(), 2);

        let task = Task {
            id: 1,
            data: "test task".to_string(),
        };

        // Two peers send same task while it's pending
        handler.on_tasks("peer1".to_string(), vec![task.clone()]);
        handler.on_tasks("peer2".to_string(), vec![task]);

        // Wait for processing
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Process completion
        if let Some((task_id, result)) = handler.pending_tasks.next().await {
            handler.on_task_completed(task_id, result.unwrap());
        }

        let reports = network.get_reports();
        // Both peers should get TASK_RECEIVED and TASK_VALID
        assert_eq!(reports.len(), 4);
        assert!(reports.contains(&("peer1".to_string(), reputation::TASK_RECEIVED)));
        assert!(reports.contains(&("peer2".to_string(), reputation::TASK_RECEIVED)));
        assert!(reports.contains(&("peer1".to_string(), reputation::TASK_VALID)));
        assert!(reports.contains(&("peer2".to_string(), reputation::TASK_VALID)));
    }

    #[tokio::test]
    async fn test_duplicate_while_pending() {
        let network = MockNetwork::new();
        let store = Arc::new(InMemoryTaskStore::new());
        let mut handler = TaskHandler::new(network.clone(), store.clone(), 2);

        let task = Task {
            id: 1,
            data: "test task".to_string(),
        };

        // Same peer sends task twice before it's processed
        handler.on_tasks("peer1".to_string(), vec![task.clone()]);
        handler.on_tasks("peer1".to_string(), vec![task]);

        let reports = network.get_reports();
        assert_eq!(reports.len(), 2);
        assert_eq!(reports[0], ("peer1".to_string(), reputation::TASK_RECEIVED));
        assert_eq!(
            reports[1],
            ("peer1".to_string(), reputation::DUPLICATE_TASK)
        );
    }

    #[tokio::test]
    async fn test_queue_full_handling() {
        let network = MockNetwork::new();
        let store = Arc::new(InMemoryTaskStore::new());

        // Create handler with very small queue
        let (queue_sender, _queue_receiver) = async_channel::bounded(1);
        let mut handler = TaskHandler {
            network: network.clone(),
            task_store: store,
            queue_sender,
            pending_tasks: FuturesUnordered::new(),
            pending_tasks_peers: HashMap::new(),
            process_timer: Box::pin(IntervalStream::new(interval(PROCESS_INTERVAL))),
        };

        // Fill the queue
        let task1 = Task {
            id: 1,
            data: "task1".to_string(),
        };
        let task2 = Task {
            id: 2,
            data: "task2".to_string(),
        };

        handler.on_tasks("peer1".to_string(), vec![task1]);
        handler.on_tasks("peer1".to_string(), vec![task2]);

        // First task should be queued, second should be dropped
        assert_eq!(handler.pending_tasks.len(), 1);
    }
}

#[tokio::main]
async fn main() {
    let network = MockNetwork::new();
    let store = Arc::new(InMemoryTaskStore::new());
    let mut handler = TaskHandler::new(network.clone(), store.clone(), 3);

    handler.on_tasks(
        "Alice".to_string(),
        vec![
            Task {
                id: 1,
                data: "Alice's task 1".to_string(),
            },
            Task {
                id: 2,
                data: "Alice's task 2".to_string(),
            },
        ],
    );

    handler.on_tasks(
        "Bob".to_string(),
        vec![
            Task {
                id: 3,
                data: "Bob's task".to_string(),
            },
            Task {
                id: 1,
                data: "Alice's task 1".to_string(),
            }, // Duplicate
        ],
    );

    handler.on_tasks(
        "Charlie".to_string(),
        vec![
            Task {
                id: 4,
                data: "invalid data".to_string(),
            }, // Invalid
        ],
    );

    println!("\n🚀 Starting handler event loop...\n");

    tokio::select! {
        _ = handler.run() => {
            println!("Handler completed");
        }
        _ = tokio::time::sleep(Duration::from_secs(2)) => {
            println!("\n⏱️  Demo timeout reached");
        }
    };

    // for _ in 0..4 {
    //     if let Some((task_id, result)) = handler.pending_tasks.next().await {
    //         if let Some(result) = result {
    //             handler.on_task_completed(task_id, result);
    //         }
    //     }
    // }

    for (peer, change) in network.get_reports() {
        println!("{}: {:+} ({})", peer, change.value, change.reason);
    }
}
