/*
    Anagram Check
    Given two strings, check if they are anagrams of each other.
    Anagrams are words or phrases formed by rearranging the letters of another,
    using all the original letters exactly once.
    The strings may contain spaces or punctuation, but you need to ignore them while checking.

    You need to implement the function `are_anagrams(s1: String, s2: String) -> bool`.
    The function should return `true` if the two strings are anagrams, and `false` otherwise.

    Hint: Consider normalizing the strings by removing non-alphabetical characters and converting to lowercase before checking.
*/

pub fn are_anagrams(s1: String, s2: String) -> bool {
  let cnt = |s: String| -> [u8; 26] {
    let mut stat = [0u8; 26];
    s.chars().filter(char::is_ascii_alphabetic).map(|c| c.to_ascii_lowercase()).for_each(|c| {
      stat[c as usize - 'a' as usize] += 1;
    });
    stat
  };

  cnt(s1) == cnt(s2)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_anagram_1() {
    let s1 = "listen".to_string();
    let s2 = "silent".to_string();
    let result = are_anagrams(s1, s2);
    println!("Are anagrams: {result}");
    assert!(result);
  }

  #[test]
  fn test_anagram_2() {
    let s1 = "evil".to_string();
    let s2 = "vile".to_string();
    let result = are_anagrams(s1, s2);
    println!("Are anagrams: {result}");
    assert!(result);
  }

  #[test]
  fn test_anagram_3() {
    let s1 = "hello".to_string();
    let s2 = "world".to_string();
    let result = are_anagrams(s1, s2);
    println!("Are anagrams: {result}");
    assert!(!result);
  }

  #[test]
  fn test_anagram_4() {
    let s1 = "Clint Eastwood".to_string();
    let s2 = "Old West Action".to_string();
    let result = are_anagrams(s1, s2);
    println!("Are anagrams: {result}");
    assert!(result);
  }

  #[test]
  fn test_anagram_5() {
    let s1 = "Astronomer".to_string();
    let s2 = "Moon starer".to_string();
    let result = are_anagrams(s1, s2);
    println!("Are anagrams: {result}");
    assert!(result);
  }
}
