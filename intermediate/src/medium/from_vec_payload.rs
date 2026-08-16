/*
  Problem 61: Intermediate — From Vec to Payload

  Implement a struct Payload { data: Vec<u8> } that implements From<Vec<u16>>.
  The conversion should pack each u16 into two u8 bytes (big-endian).

  Run the tests for this problem with:
    cargo test --test from_vec_payload_test
*/

#[derive(Debug, PartialEq)]
pub struct Payload {
    pub data: Vec<u8>,
}

impl From<Vec<u16>> for Payload {
    fn from(v: Vec<u16>) -> Self {
        let mut vec_8 = Vec::new();
        for vec in v {
            let upper = ((vec >> 8) & 255u16) as u8;
            let lower = (vec & 255u16) as u8;
            vec_8.push(upper);
            vec_8.push(lower);
        }
        Self { data: vec_8 }
    }
}
