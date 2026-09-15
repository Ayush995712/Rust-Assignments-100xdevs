/*
  Problem 100: Async Task Scheduler

  Implement a TaskScheduler that can schedule async closures to run after
  a specific delay. It should use a PriorityQueue (or sorted Vec) to keep track
  of tasks and a background tokio task to execute them when their time comes.

  Run the tests for this problem with:
    cargo test --test task_scheduler_test
*/

use tokio::time::{sleep, Duration, Instant};
use tokio::sync::Notify;
use std::sync::{Arc, Mutex};

pub struct TaskScheduler {
    pub tasks: Arc<Mutex<Vec<(Instant, Box<dyn FnOnce() + Send + 'static>)>>>,
    pub notify: Arc<Notify>
}

impl TaskScheduler {
    pub fn new() -> Self {
        Self { 
            tasks: Arc::new(Mutex::new(Vec::new())),
            notify: Arc::new(Notify::new())
        }
    }

    pub fn schedule<F>(&self, delay: Duration, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let execute_at = Instant::now() + delay;
        let mut tasks = self.tasks.lock().unwrap();

        for (i, (instant, _)) in tasks.iter().enumerate() {
            if *instant >= execute_at {
                tasks.insert(i, (execute_at, Box::new(f)));
                self.notify.notify_one();
                return;
            }
        }

        tasks.push((execute_at, Box::new(f)));
        self.notify.notify_one();
    }

    pub async fn start(&self) {
        let notify = Notify::new();
        loop {
            if self.tasks.lock().unwrap().is_empty() {
                notify.notified().await;
            };

            let (instant_to_execute, closure_to_execute) = {
                let mut task_to_execute = self.tasks.lock().unwrap();
                task_to_execute.remove(0)
            };

            let duration_to_wait = instant_to_execute - Instant::now();
            sleep(duration_to_wait).await;
            closure_to_execute();
        }
    }
}
