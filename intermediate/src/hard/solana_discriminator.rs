/*
  Problem 53: Solana-Style Account Data — Discriminator

  Simulate a Solana-style account data structure. Define a trait AccountData
  with methods discriminator() -> [u8; 8], serialize(&self) -> Vec<u8> and
  deserialize(data) -> Result<Self, String>. Implement it for TokenAccount
  { owner: [u8; 32], amount: u64 }. The serialized format is
  [discriminator: 8 bytes][owner: 32 bytes][amount: 8 bytes LE].

  Run the tests for this problem with:
    cargo test --test solana_discriminator_test
*/

pub trait AccountData: Sized {
    fn discriminator() -> [u8; 8];
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8]) -> Result<Self, String>;
}

#[derive(Debug, PartialEq)]
pub struct TokenAccount {
    pub owner: [u8; 32],
    pub amount: u64,
}

impl AccountData for TokenAccount {
    fn discriminator() -> [u8; 8] {
        // Use a fixed discriminator: "TOKENACC"
        [0x54, 0x4f, 0x4b, 0x45, 0x4e, 0x41, 0x43, 0x43]
    }

    fn serialize(&self) -> Vec<u8> {
        let mut vec_acc = Vec::new();
        let dis = TokenAccount::discriminator();
        for i in dis {
            vec_acc.push(i);
        };
        for i in self.owner {
            vec_acc.push(i);
        };
        for i in self.amount.to_le_bytes() {
            vec_acc.push(i);
        };
        vec_acc
    }

    fn deserialize(data: &[u8]) -> Result<Self, String> {
        if data.len() != 48 {
            return Err("Length should be 48".to_string())
        }

        for (i, j) in data[..8].iter().enumerate() {
            if TokenAccount::discriminator()[i] != *j {
                return Err("Deserialized data does not match with TokenAccount::Discriminator()".to_string());
            }
        }

        let mut owner = [0u8; 32];
        owner.copy_from_slice(&data[8..40]);

        let amount = u64::from_le_bytes(data[40..48].try_into().map_err(|_| "Invalid amount length".to_string())?);
         Ok(Self {
                owner,
                amount,
         })
    }
}
