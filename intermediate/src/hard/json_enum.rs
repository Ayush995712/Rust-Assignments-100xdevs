/*
  Problem 65: Data Processing — JSON-like Enum

  Define an enum Value representing a simplified JSON value:
  Null, Bool(bool), Number(f64), String(String), Array(Vec<Value>),
  and Object(HashMap<String, Value>). Implement a method fn to_json_string(&self) -> String
  that produces a JSON-formatted string.

  Run the tests for this problem with:
    cargo test --test json_enum_test
*/

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl Value {
    pub fn to_json_string(&self) -> String {
        match self {
            Value::Null => "null".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Number(num) => num.to_string(),
            Value::String(str) => format!("\"{}\"", str),
            Value::Array(values) => {
                let items: Vec<String> = values.iter().map(|val| val.to_json_string()).collect();
                format!("[{}]", items.join(", "))
            },
            Value::Object(hash) => {
                let mut vec_pair = Vec::new();
                for (key, value) in hash {
                    let pair = format!("\"{}\": {}", key, value.to_json_string());
                    vec_pair.push(pair);
                };
                format!("{{{}}}", vec_pair.join(","))
            }
        }
    }
}
