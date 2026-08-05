/*
  Problem 20: Unique Elements

  Write a function that takes a Vec<i32> and returns a Vec<i32> containing only unique elements,
  preserving their first occurrence order. Use a HashSet for tracking seen elements.

  Run the tests for this problem with:
    cargo test --test unique_elements_test
*/

use std::collections::HashSet;

pub fn unique_elements(v: Vec<i32>) -> Vec<i32> {
  let mut set = HashSet::new();
  let mut unique_ele = Vec::new();
    for element in v {
      if !set.contains(&element) {
        set.insert(element);
        unique_ele.push(element);
      }
    }
    return unique_ele
}
