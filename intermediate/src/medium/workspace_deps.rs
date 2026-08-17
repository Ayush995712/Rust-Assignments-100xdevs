/*
  Problem 64: Workspace Dependencies

  Create a function that simulates a cross-crate dependency check.
  Write a function is_crate_active(crate_name: &str) -> bool that returns
  true if the crate name is "fundamentals", "intermediate",
  or "async".

  Run the tests for this problem with:
    cargo test --test workspace_deps_test
*/

pub fn is_crate_active(crate_name: &str) -> bool {
    if crate_name == "fundamentals" || crate_name == "intermediate" || crate_name == "async" {
      return true
    } else {
      return false
    }
}
