/*
  Problem 81: Parallel Word Count

  Write a function that takes a Vec<String> (lines of text) and counts the
  occurrences of words in parallel. Split the lines among 4 threads. Each thread
  computes a local HashMap, and then the main thread merges them into a final
  HashMap<String, usize>.

  Run the tests for this problem with:
    cargo test --test parallel_word_count_test
*/

use std::collections::HashMap;
use std::thread;

pub fn parallel_word_count(lines: Vec<String>) -> HashMap<String, usize> {
  if lines.is_empty() {
    return HashMap::new()
  }
    let num_threads = 4;
    let mut handles = Vec::new();

    let chunk_size = (lines.len() + num_threads - 1) / num_threads;

    for chunk in lines.chunks(chunk_size) {
        let chunk = chunk.to_vec();

        let handle = thread::spawn(move || {
            let mut local_map = HashMap::new();

            for line in chunk {
                for word in line.split_whitespace() {
                    *local_map.entry(word.to_string()).or_insert(0) += 1;
                }
            }

            local_map
        });

        handles.push(handle);
    }

    let mut final_map = HashMap::new();

    for handle in handles {
        let local_map = handle.join().unwrap();

        for (word, count) in local_map {
            *final_map.entry(word).or_insert(0) += count;
        }
    }

    final_map
}
