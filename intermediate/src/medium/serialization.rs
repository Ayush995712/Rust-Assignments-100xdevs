/*
  Problem 47: Serialization — Manual to_bytes / from_bytes

  Define a struct Record { id: u32, value: u16 }. Implement methods
  to_bytes(&self) -> Vec<u8> and from_bytes(data: &[u8]) -> Result<Self, String>
  using little-endian byte order. The serialized format should be
  [id: 4 bytes][value: 2 bytes] = 6 bytes total.

  Run the tests for this problem with:
    cargo test --test serialization_test
*/

#[derive(Debug, PartialEq)]
pub struct Record {
    pub id: u32,
    pub value: u16,
}

impl Record {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut v = Vec::new();

        let lower_byte_id = (self.id & 255u32) as u8;
        let lowerm_byte_id = ((self.id >> 8) & 255u32) as u8;
        let upper_byte_id = (self.id >> 24 & 255u32) as u8;
        let upperm_byte_id = ((self.id >> 16) & 255u32) as u8;

        let lower_byte_value = (self.value & 255u16) as u8;
        let upper_byte_value = (self.value >> 8 & 255u16) as u8;

        v.push(lower_byte_id);
        v.push(lowerm_byte_id);
        v.push(upperm_byte_id);
        v.push(upper_byte_id);

        v.push(lower_byte_value);
        v.push(upper_byte_value);

        return v
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        if data.len() != 6 {
            return Err("Invalid data length".to_string())
        }

        let mut id = 0 as u32;
        let mut value = 0u16;
        
        for (i, byte) in data.iter().enumerate() {
            if i < 4 {
                let rotation = 8 * i;
                let id_dup = (*byte as u32) << rotation;
                id |= id_dup;
            } else {
                let rotation = 8 * (i - 4);
                let val_dup = (*byte as u16) << rotation;
                value |= val_dup;
            }
        }
        Ok( Self { id, value })
    }
}
