/*
  Problem 92: Async Retry Logic

  Write an async function retry_operation<F, Fut, T, E>(f: F, max_retries: usize)
  -> Result<T, E> where F is a factory that produces a future Fut.
  If the future returns an Err, retry up to max_retries times before giving up.
  Wait 10ms between retries.

  Run the tests for this problem with:
    cargo test --test retry_logic_test
*/

use tokio::time::{sleep, Duration};
use std::future::Future;

pub async fn retry_operation<F, Fut, T, E>(mut f: F, max_retries: usize) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    match f().await {
        Ok(res) => return Ok(res),
        Err(mut err) => {
            for i in 0..max_retries {
                sleep(Duration::from_millis(10)).await;
                match f().await {
                    Ok(res) => return Ok(res),
                    Err(e) => { err = e; if i == max_retries - 1 { return Err(err)} }
                };
            }
            Err(err)
        }
    }
}
