use std::collections::HashSet;

pub fn new_count_distinct(str: &str) -> usize {
  str.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect::<HashSet<_>>().len()
}
