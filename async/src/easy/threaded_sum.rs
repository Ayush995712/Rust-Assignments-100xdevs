/*
  Problem 71: Multithreaded Sum

  Write a function that takes a Vec<i32> and splits it into two halves.
  Sum each half in a separate thread using std::thread::spawn and return
  the total sum.

  Run the tests for this problem with:
    cargo test --test threaded_sum_test
*/

use std::{thread};

pub fn threaded_sum(v: Vec<i32>) -> i32 {
    let length = v.len();
    let vec1 = v[..length/2].to_vec();
    let vec2 = v[length/2..].to_vec();
    let handle1 = thread::spawn(move || {
      vec1.iter().sum::<i32>()
    });
    let handle2 = thread::spawn(move || {
      vec2.iter().sum::<i32>()
    });
    let sum1 = handle1.join().unwrap();
    let sum2 = handle2.join().unwrap();
    sum1 + sum2
}
