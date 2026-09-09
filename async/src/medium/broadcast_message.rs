/*
  Problem 89: Message Passing — Broadcast Channel

  Write an async function broadcast_demo() that creates a
  tokio::sync::broadcast channel. Spawn 3 tasks that each subscribe to the
  channel and receive a message. The main function should send a message
  and ensure all tasks receive it. Return the sum of received values.

  Run the tests for this problem with:
    cargo test --test broadcast_message_test
*/

use tokio::sync::broadcast;

pub async fn broadcast_demo() -> i32 {
    let (tx, _rx) = broadcast::channel(10);
    let mut rx2 = tx.subscribe();
    let mut rx3 = tx.subscribe();
    let mut rx4 = tx.subscribe();

    let res2 = tokio::spawn(async move {
      let res: i32 = rx2. recv().await.unwrap();
      res
    });
    let res3 = tokio::spawn(async move {
      let res: i32 = rx3. recv().await.unwrap();
      res
    });
    let res4 = tokio::spawn(async move {
      let res: i32 = rx4. recv().await.unwrap();
      res
    });

    tx.send(10).unwrap();

    res2.await.unwrap() + res3.await.unwrap() + res4.await.unwrap()
    
}
