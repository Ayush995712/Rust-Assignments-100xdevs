/*
  Problem 50: Byte Manipulation — Pack Two u16 into u32

  Write a function pack_u16(high, low) that packs two u16 values into a single u32
  (high in upper 16 bits, low in lower 16 bits). Write a companion unpack_u32(packed)
  that reverses the operation.

  Run the tests for this problem with:
    cargo test --test pack_u16_test
*/

pub fn pack_u16(high: u16, low: u16) -> u32 {
    let pack = high as u32;
    let pack_s = pack << 16;
    pack_s | (low as u32)
}

pub fn unpack_u32(packed: u32) -> (u16, u16) {
    let high = ((packed >> 16) & 0xFFFFu32) as u16;
    let low = (packed & 0xFFFFu32) as u16;
    (high, low)
}
