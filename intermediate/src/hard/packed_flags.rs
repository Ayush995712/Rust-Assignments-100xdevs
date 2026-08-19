/*
  Problem 54: Data Packing — Bitfield Struct

  Define a struct PackedFlags that stores 8 boolean flags in a single u8.
  Implement methods new(), set(index, value), get(index), and as_byte().
  Index ranges from 0–7.

  Run the tests for this problem with:
    cargo test --test packed_flags_test
*/

#[derive(Debug, PartialEq)]
pub struct PackedFlags {
    pub bits: u8,
}

impl PackedFlags {
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn set(&mut self, index: u8, value: bool) {
        if value {
            let rot = 1u8 << index;
            self.bits |= rot;
        } else {
            let rot = !(1u8 << index);
            self.bits &= rot;
        }
    }

    pub fn get(&self, index: u8) -> bool {
        let rot = 1u8 << index;
        if (rot & self.bits) == 0u8 {
            return false
        } else {
            return true
        }
    }

    pub fn as_byte(&self) -> u8 {
        self.bits
    }
}
