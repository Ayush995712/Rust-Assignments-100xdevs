/*
  Problem 68: Bump Allocator — Simple Simulation

  Simulate a bump allocator. Define a struct BumpAllocator with a buffer [u8; 1024]
  and a cursor (next available byte). Implement alloc(size: usize) -> Result<&mut [u8], String>
  that returns a slice of the given size and advances the cursor.

  Run the tests for this problem with:
    cargo test --test bump_allocator_test
*/

pub struct BumpAllocator {
    pub buffer: [u8; 1024],
    pub cursor: usize,
}

impl BumpAllocator {
    pub fn new() -> Self {
        Self { buffer: [0; 1024], cursor: 0 }
    }

    pub fn alloc(&mut self, size: usize) -> Result<&mut [u8], String> {
        if size > (self.buffer.len() - self.cursor)  {
            return Err("available size is less than what you demand".to_string());
        }

        let c = self.cursor;
        self.cursor += size;
        Ok(&mut self.buffer[c..self.cursor])
    }
}
