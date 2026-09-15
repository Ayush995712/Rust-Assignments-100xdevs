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

    pub fn start(&self) {
        let tasks = Arc::clone(&self.tasks);
        let notify = Arc::clone(&self.notify);

        tokio::spawn(async move {
            loop {
                let next_time = {
                    let guard = tasks.lock().unwrap();
                    guard.first().map(|(t, _)| *t)
                };

                match next_time {
                    Some(execute_at) => {
                        let now = Instant::now();
                        if execute_at <= now {
                            let task = {
                                let mut guard = tasks.lock().unwrap();
                                guard.remove(0)
                            };
                            (task.1)();
                        } else {
                            tokio::select! {
                                _ = sleep(execute_at - now) => {}
                                _ = notify.notified() => {}
                            }
                        }
                    }
                    None => {
                        notify.notified().await;
                    }
                }
            }
        });
    }
}
