/*
  Problem 43: Type Alias — Result Shorthand

  Define a type alias AppResult<T> = Result<T, AppError> and a custom error enum
  AppError with variants NotFound(String) and ParseFailed(String).
  Write two functions: find_user(id) and parse_age(input), both returning AppResult<T>.

  Run the tests for this problem with:
    cargo test --test type_alias_result_test
*/

#[derive(Debug, PartialEq)]
pub enum AppError {
    NotFound(String),
    ParseFailed(String),
}

pub type AppResult<T> = Result<T, AppError>;

pub fn find_user(id: u32) -> AppResult<String> {
    // Return Ok("User_<id>") if id > 0, else NotFound
    if id > 0 {
        Ok(format!("User_{}", id))
    } else {
        Err(AppError::NotFound(format!("User {} not found", id)))
    }
}

pub fn parse_age(input: &str) -> AppResult<u32> {
    // Parse the string as u32, return ParseFailed on error
    let parsed_input = input.parse::<u32>();
    match parsed_input {
        Ok(value) => Ok(value),
        Err(e) => Err(AppError::ParseFailed(e.to_string()))
    }
}
