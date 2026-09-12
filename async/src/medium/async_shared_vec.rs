/*
  Problem 91: Async Shared State — Mutex Vec

  Rewrite Problem 74 / 79 using tokio::sync::Mutex to share a Vec<i32>.
  Spawn 5 tasks, each pushing 10 numbers into the vector.
  Return the length of the final vector.

  Run the tests for this problem with:
    cargo test --test async_shared_vec_test
*/

use tokio::sync::Mutex;
use std::sync::Arc;

pub async fn async_shared_vec() -> usize {
  let check_vec = Arc::new(Mutex::new(vec![]));
  let mut handles = vec![];

  for _ in 0..5 {
    let cloned_vec = Arc::clone(&check_vec);
    let handle = tokio::spawn(async move {
      let mut lock_cloned_vec = cloned_vec.lock().await;
      for i in 0..10 {
        lock_cloned_vec.push(i);
      }
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.await.unwrap()
  }

  let result = check_vec.lock().await.len();
  result
}
