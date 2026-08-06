/*
  Problem 24: Vec — Running Sum

  Write a function that takes a Vec<i32> and returns a new Vec<i32> where each element is the running sum up to that index.
  For example, [1, 2, 3] becomes [1, 3, 6].

  Run the tests for this problem with:
    cargo test --test running_sum_test
*/

pub fn running_sum(v: Vec<i32>) -> Vec<i32> {
    let mut sum = 0;
    let mut sum_vec = Vec::new();
    for i in &v {
      sum += i;
      sum_vec.push(sum);
    };
    return sum_vec
}
