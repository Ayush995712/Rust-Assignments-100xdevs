/*
  Problem 80: Shared State — Cancellable Worker

  Write a function that spawns a thread which runs an infinite loop.
  The thread should check a shared atomic flag (AtomicBool) periodically.
  If the flag is set to true, the thread should exit.
  The main thread should sleep for 50ms, then set the flag to true.

  Run the tests for this problem with:
    cargo test --test cancellable_worker_test
*/

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn cancellable_worker() {
  let some_bool = Arc::new(AtomicBool::new(false));
  let clone_some_bool = Arc::clone(&some_bool);

  thread::spawn(move || {
    loop {
      thread::sleep(Duration::from_millis(10));
      if clone_some_bool.load(Ordering::Relaxed) { break };
    }
  });

  thread::sleep(Duration::from_millis(50));
  some_bool.store(true, Ordering::Relaxed);
}
