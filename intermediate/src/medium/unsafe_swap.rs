/*
  Problem 45: Unsafe — Swap with Raw Pointers

  Write a function that takes two mutable references to i32 values and swaps them
  using raw pointers inside an unsafe block. Do not use std::mem::swap.

  Run the tests for this problem with:
    cargo test --test unsafe_swap_test
*/

pub fn unsafe_swap(a: &mut i32, b: &mut i32) {
    let first = a as *mut i32;
    let second = b as *mut i32;
    unsafe {
      let swap = *first;
      *first = *second;
      *second = swap
    }
}
