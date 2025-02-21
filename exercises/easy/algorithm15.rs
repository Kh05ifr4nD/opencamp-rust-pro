/*
    Longest Substring Without Repeating Characters
    Given a string, find the length of the longest substring without repeating characters.
    The substring must not contain any duplicate characters, and its length should be maximized.

    You need to implement the function `longest_substring_without_repeating_chars(s: String) -> i32`.
    The function should return the length of the longest substring without repeating characters.

    Hint: Consider using the sliding window technique to efficiently solve this problem in O(n) time complexity.
*/

pub fn longest_substring_without_repeating_chars(s: &String) -> i32 {
  let mut arr_pos_last = [-1; 256];
  let mut len_max = 0;
  let mut l = 0;

  for (r, &c) in s.as_bytes().iter().enumerate() {
    let c = c as usize;

    if arr_pos_last[c] >= l as i32 {
      l = (arr_pos_last[c] + 1) as usize;
    }

    arr_pos_last[c] = r as i32;

    len_max = len_max.max((r - l + 1) as i32);
  }

  len_max
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_longest_substring_1() {
    let s = "abcabcbb".to_string();
    let result = longest_substring_without_repeating_chars(&s);
    println!("Length of longest substring: {result}");
    assert_eq!(result, 3); // "abc"
  }

  #[test]
  fn test_longest_substring_2() {
    let s = "bbbbb".to_string();
    let result = longest_substring_without_repeating_chars(&s);
    println!("Length of longest substring: {result}");
    assert_eq!(result, 1); // "b"
  }

  #[test]
  fn test_longest_substring_3() {
    let s = "pwwkew".to_string();
    let result = longest_substring_without_repeating_chars(&s);
    println!("Length of longest substring: {result}");
    assert_eq!(result, 3); // "wke"
  }

  #[test]
  fn test_longest_substring_4() {
    let s = String::new();
    let result = longest_substring_without_repeating_chars(&s);
    println!("Length of longest substring: {result}");
    assert_eq!(result, 0); // Empty string
  }

  #[test]
  fn test_longest_substring_5() {
    let s = "abcde".to_string();
    let result = longest_substring_without_repeating_chars(&s);
    println!("Length of longest substring: {result}");
    assert_eq!(result, 5); // "abcde"
  }
}
