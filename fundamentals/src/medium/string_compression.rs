/*
  Problem 33: String Compression

  Write a function that performs basic string compression using the counts of repeated characters.
  For example, "aabcccccaaa" becomes "a2b1c5a3".
  If the compressed string is not shorter than the original, return the original string.

  Run the tests for this problem with:
    cargo test --test string_compression_test
*/

use std::format;

pub fn compress(s: &str) -> String {
   if s.is_empty() { return String::new() };

   let mut compressed_string = String::new();
   let mut count = 1;
   let mut current_char = s.chars().next().unwrap();

   for c in s[1..].chars() {
    if current_char == c {
      count += 1;
    } else {
      compressed_string.push_str(&format!("{}{}", current_char, count));
      current_char = c;
      count = 1;
    }
   }
   compressed_string.push_str(&format!("{}{}", current_char, count));
   if compressed_string.len() >= s.len() {
     return s.to_string() 
   } 
   else {
    return compressed_string
   }
}
