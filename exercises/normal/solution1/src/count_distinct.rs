use std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
    let table :HashSet<&str>= input_str.split(',').collect();
    table.len()
}
