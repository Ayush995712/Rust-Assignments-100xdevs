/*
  Problem 35: Mini Phonebook

  Build a Phonebook struct backed by a HashMap<String, String> (name -> phone).
  Implement methods new, add, lookup, and remove.
  Also implement the Display trait to print all entries sorted by name, one per line
  as "Name: Phone".

  Run the tests for this problem with:
    cargo test --test mini_phonebook_test
*/

use std::collections::HashMap;
use std::fmt::{self};

pub struct Phonebook {
    pub entries: HashMap<String, String>,
}

impl Phonebook {
    pub fn new() -> Self {
        Self { entries: HashMap::new() }
    }

    pub fn add(&mut self, name: &str, phone: &str) {
        self.entries.insert(name.to_string(), phone.to_string());
    }

    pub fn lookup(&self, name: &str) -> Option<&String> {
        let found = self.entries.get(name);
        return found
    }

    pub fn remove(&mut self, name: &str) -> bool {
        let removed = self.entries.remove(name).is_some();
        return removed
    }
}

impl fmt::Display for Phonebook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut names: Vec<_> = self.entries.iter().collect();
        names.sort_by_key(|name| name.0);
        for ( name, phone ) in names {
            writeln!(f, "{}: {}", name, phone)?;
        }
        Ok(())
    }
}
