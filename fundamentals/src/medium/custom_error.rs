/*
  Problem 26: Custom Error Type

  Define a custom error enum ValidationError with variants TooShort, TooLong,
  and InvalidChar(char). Write a function that validates a username:
  must be 3–20 characters and only contain alphanumeric chars or underscores.
  Return Ok(()) or the appropriate error.

  Run the tests for this problem with:
    cargo test --test custom_error_test
*/

use crate::medium::custom_error::ValidationError::{InvalidChar, TooLong, TooShort};

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    TooShort,
    TooLong,
    InvalidChar(char),
}

pub fn validate_username(username: &str) -> Result<(), ValidationError> {
    if username.len() < 3 {
        return Err(TooShort)
    } else if username.len() > 20 {
        return Err(TooLong)
    };
    for c in username.chars() {
        if !(c.is_alphanumeric() || c == '_') {
            return Err(InvalidChar(c))
        }
    };
    return Ok(())
}
