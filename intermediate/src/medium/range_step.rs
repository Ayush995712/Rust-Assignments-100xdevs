/*
  Problem 38: Custom Iterator — Range Step

  Implement a custom iterator RangeStep that yields values from start to end (exclusive)
  with a given step. All values are i32. If step is zero or the range is invalid,
  the iterator should yield nothing.

  Run the tests for this problem with:
    cargo test --test range_step_test
*/

pub struct RangeStep {
    pub current: i32,
    pub end: i32,
    pub step: i32,
}

impl RangeStep {
    pub fn new(start: i32, end: i32, step: i32) -> Self {
        RangeStep { current: start, end: end, step: step }
    }
}

impl Iterator for RangeStep {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step == 0 || self.current == self.end {
            return None
        } else if (self.step < 0 && self.end > self.current) || (self.step > 0 && self.end < self.current) {
            return None
        } else {
            let value = self.current;
            self.current += self.step;
            Some(value)
        }
    }
}
