/*
  Problem 78: Thread Pool — Simple Worker Thread

  Implement a simple ThreadPool that can execute closures. The pool should
  maintain a fixed number of worker threads and use a channel to send
  jobs to workers. Implement new(size: usize) and execute<F>(&self, f: F).

  Run the tests for this problem with:
    cargo test --test thread_pool_test
*/

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    pub workers: Vec<Worker>,
    pub sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let mut worker_vec = Vec::new();
        let (sender, receiver) = mpsc::channel::<Job>();
        let shareable_receiver = Arc::new(Mutex::new(receiver));

        for s in 0..size {
            let clone_receiver = Arc::clone(&shareable_receiver);
            let handle = thread::spawn(move || {
                loop {
                    let lock_receiver = clone_receiver.lock().unwrap();
                    let job_to_do = lock_receiver.recv();
                    drop(lock_receiver);
                    
                    match job_to_do {
                        Ok(job) => job(),
                        Err(_) => break,
                    }
                }
            });
            worker_vec.push(Worker { id: s, thread: Some(handle) });
        };
        Self { workers: worker_vec, sender: sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap()
    }
}

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}
