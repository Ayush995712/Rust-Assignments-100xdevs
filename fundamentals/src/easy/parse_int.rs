/*
  Problem 15: Parse Integer with Result

  Write a function that takes a &str and attempts to parse it into an i32.
  Return Ok(value) on success, or Err(String) with a descriptive error message on failure.

  Run the tests for this problem with:
    cargo test --test parse_int_test
*/
use std::format;

pub fn parse_int(s: &str) -> Result<i32, String> {
    return match s.parse() {
      Ok(parsed_int) => Ok(parsed_int),
      Err(error) => Err(format!("Problem parsing the string to int: {error}")), 
    }
}
