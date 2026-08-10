/*
  Problem 34: Matrix Transpose

  Write a function that takes a Vec<Vec<i32>> representing a matrix and returns its transpose.
  The transpose of a matrix swaps rows and columns. Assume the input is a valid rectangular
  matrix (all rows have the same length). Return an empty vec for empty input.

  Run the tests for this problem with:
    cargo test --test matrix_transpose_test
*/

pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
  if matrix.is_empty() {
    return Vec::new()
  }

  let number_of_element = matrix[0].len();
  let mut result = Vec::new();

  for _i in 0..number_of_element {
    result.push(Vec::new());
  };

  for mat in matrix {
    for (j, ve) in mat.into_iter().enumerate() {
      result[j].push(ve);
    }
  };
  return result
}
