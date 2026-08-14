/*
  Problem 40: Rc<T> — Shared Ownership

  Create a function that takes a String value, wraps it in Rc<String>,
  clones the Rc twice, and returns a tuple of the strong reference count
  and the string value itself (cloned for the return).
  This exercises shared ownership without copying the underlying data.

  Run the tests for this problem with:
    cargo test --test rc_shared_test
*/

use std::rc::Rc;

pub fn shared_ownership(value: String) -> (usize, String) {
    let rc_string = Rc::new(value);
    let clone1 = Rc::clone(&rc_string);
    let clone2 = Rc::clone(&rc_string);
    (Rc::strong_count(&rc_string), (*rc_string).clone())
}
