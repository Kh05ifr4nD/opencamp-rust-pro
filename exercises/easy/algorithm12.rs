/*
    Palindrome Check
    Given a string, check if it is a palindrome (i.e., it reads the same forward and backward).
    The solution should ignore case differences and non-alphabetical characters.

    You need to implement the function `is_palindrome(s: String) -> bool`.
    The function should return `true` if the string is a palindrome, and `false` otherwise.

    Hint: Consider normalizing the string by converting it to lowercase and removing non-alphabetical characters before checking.
*/

pub fn is_palindrome(s: &str) -> bool {
  let str: Vec<char> = s.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect();

  let mut l = 0;
  let mut r = str.len().saturating_sub(1);

  while l < r {
    if str[l] != str[r] {
      return false;
    }
    l += 1;
    r -= 1;
  }
  true
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_palindrome_1() {
    let s = "A man, a plan, a canal, Panama".to_string();
    let result = is_palindrome(&s);
    println!("Is palindrome: {result}");
    assert!(result);
  }

  #[test]
  fn test_palindrome_2() {
    let s = "Racecar".to_string();
    let result = is_palindrome(&s);
    println!("Is palindrome: {result}");
    assert!(result);
  }

  #[test]
  fn test_palindrome_3() {
    let s = "Hello, World!".to_string();
    let result = is_palindrome(&s);
    println!("Is palindrome: {result}");
    assert!(!result);
  }

  #[test]
  fn test_palindrome_4() {
    let s = "No 'x' in Nixon".to_string();
    let result = is_palindrome(&s);
    println!("Is palindrome: {result}");
    assert!(result);
  }

  #[test]
  fn test_palindrome_5() {
    let s = "Was it a car or a cat I saw?".to_string();
    let result = is_palindrome(&s);
    println!("Is palindrome: {result}");
    assert!(result);
  }
}
