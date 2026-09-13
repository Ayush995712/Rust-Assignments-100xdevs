/*
  Problem 79: Atomic Counter

  Rewrite the multithreaded counter problem (Problem 74) using AtomicI32
  instead of Mutex<i32>. Compare the performance and complexity.
  Show use of fetch_add and Ordering.

  Run the tests for this problem with:
    cargo test --test atomic_counter_test
*/

use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::thread;

pub fn atomic_counter() -> i32 {
    let atomic_value = Arc::new(AtomicI32::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
      let clone_atomic_value = Arc::clone(&atomic_value);
      let handle = thread::spawn(move || {
        clone_atomic_value.fetch_add(100, Ordering::SeqCst);
      });
      handles.push(handle);
    }

    for handle in handles {
      handle.join().unwrap();
    }

    atomic_value.load(Ordering::Relaxed)
}
