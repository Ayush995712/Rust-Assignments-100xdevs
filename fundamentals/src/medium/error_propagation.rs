/*
  Problem 25: Error Propagation with ?

  Write a function parse_and_add that takes two string slices,
  parses each into an i32, and returns their sum.
  Use the ? operator to propagate parse errors.
  Return Result<i32, std::num::ParseIntError>.

  Run the tests for this problem with:
    cargo test --test error_propagation_test
*/

pub fn parse_and_add(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    let parsed_a: i32 = a.parse()?;
    let parsed_b: i32 = b.parse()?;
    return Ok(parsed_a + parsed_b)
}
