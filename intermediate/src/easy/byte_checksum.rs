/*
  Problem 59: Byte Checksum Utility

  Write a function compute_checksum(data: &[u8]) -> u8 that returns the XOR of all
  bytes in the input. If the input is empty, return 0.

  Run the tests for this problem with:
    cargo test --test byte_checksum_test
*/

pub fn compute_checksum(data: &[u8]) -> u8 {
    if data.is_empty() { return 0 };

    let mut fir = data[0];

    for d in &data[1..] {
      fir ^= *d;
    };
    return fir
}

/* More idiomatic way

  pub fn compute_checksum(data: &[u8]) -> u8 {
    data.iter().fold(0, |acc, &byte| acc ^ byte)
}
    
*/
