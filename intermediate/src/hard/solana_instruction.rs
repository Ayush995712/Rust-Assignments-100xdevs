/*
  Problem 60: Solana-Style Instruction — Unpack

  Simulate unpacking a Solana instruction. Define an enum Instruction with variants
  Initialize, Mint { amount: u64 }, and Transfer { amount: u64 }.
  Write a function unpack(data: &[u8]) -> Result<Instruction, String>.
  Data format: [tag: 1 byte][data: remaining bytes LE].
  Tags: 0 = Initialize, 1 = Mint, 2 = Transfer.

  Run the tests for this problem with:
    cargo test --test solana_instruction_test
*/

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Initialize,
    Mint { amount: u64 },
    Transfer { amount: u64 },
}

pub fn unpack(data: &[u8]) -> Result<Instruction, String> {
    if data.is_empty() {
        return Err("Data can't be empty".to_string())
    }

    let tag = data[0];
    if tag == 0u8 {
        return Ok(Instruction::Initialize);
    } else if tag == 1u8 {
        if data.len() != 9 { return Err("Mint amount needs to be 8 bytes".to_string())};
        return Ok(Instruction::Mint { amount: u64::from_le_bytes(data[1..].try_into().map_err(|_| "Invalid amount bytes".to_string())?)})
    } else if tag == 2u8 {
        if data.len() != 9 { return Err("Transfer amount needs to be 8 bytes".to_string())};
        return Ok(Instruction::Transfer { amount: u64::from_le_bytes(data[1..].try_into().map_err(|_| "Invalid amount bytes".to_string())?) })
    } else {
        return Err("Data is not of Instruction type".to_string());
    }
}
