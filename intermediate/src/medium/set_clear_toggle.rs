/*
  Problem 49: Bitwise — Set, Clear, Toggle Bits

  Write three functions: set_bit(value, bit), clear_bit(value, bit),
  and toggle_bit(value, bit). Bit positions are 0-indexed from the LSB.
  All functions take u32 value and u8 bit position, and return the modified u32.

  Run the tests for this problem with:
    cargo test --test set_clear_toggle_test
*/

pub fn set_bit(value: u32, bit: u8) -> u32 {
    let bit_position = 1u32 << bit;
    value | bit_position
}

pub fn clear_bit(value: u32, bit: u8) -> u32 {
    let bit_position = !(1u32 << bit);
    value & bit_position 
}

pub fn toggle_bit(value: u32, bit: u8) -> u32 {
    let bit_position = 1u32 << bit;
    value ^ bit_position  // XOR operation

    // another approach
    // if value & bit_position != 0u32 {
    //     clear_bit(value, bit)
    // } else {
    //     set_bit(value, bit)
    // }
}
