/*
  stack
  This question requires you to use a stack to achieve a bracket match
*/

#[derive(Debug)]
struct Stack<T> {
  size: usize,
  data: Vec<T>,
}

impl<T> Stack<T> {
  fn new() -> Self {
    Self { size: 0, data: Vec::new() }
  }
  fn empty(&self) -> bool {
    0 == self.size
  }
  fn size(&self) -> usize {
    self.size
  }
  fn clear(&mut self) {
    self.size = 0;
    self.data.clear();
  }
  fn push(&mut self, val: T) {
    self.data.push(val);
    self.size += 1;
  }
  fn pop(&mut self) -> Option<T> {
    let val = self.data.pop();
    if val.is_some() {
      self.size -= 1;
    }
    val
  }
  fn top(&self) -> Option<&T> {
    if 0 == self.size {
      None
    } else {
      self.data.get(self.size - 1)
    }
  }
  fn top_mut(&mut self) -> Option<&mut T> {
    if 0 == self.size {
      None
    } else {
      self.data.get_mut(self.size - 1)
    }
  }
  fn into_iter(self) -> IntoIter<T> {
    IntoIter(self)
  }
  fn iter(&self) -> Iter<'_, T> {
    Iter { stack: self.data.iter().collect() }
  }
  fn iter_mut(&mut self) -> IterMut<'_, T> {
    IterMut { stack: self.data.iter_mut().collect() }
  }
}

struct IntoIter<T>(Stack<T>);

impl<T: Clone> Iterator for IntoIter<T> {
  type Item = T;
  fn next(&mut self) -> Option<Self::Item> {
    if self.0.empty() {
      None
    } else {
      self.0.size -= 1;
      self.0.data.pop()
    }
  }
}

struct Iter<'a, T> {
  stack: Vec<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;
  fn next(&mut self) -> Option<Self::Item> {
    self.stack.pop()
  }
}

struct IterMut<'a, T> {
  stack: Vec<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;
  fn next(&mut self) -> Option<Self::Item> {
    self.stack.pop()
  }
}

fn brkt_match(brkt: &str) -> bool {
  let mut stack = Stack::new();
  for c in brkt.chars() {
    match c {
      '(' | '{' | '[' => stack.push(c),
      ')' => {
        if stack.pop() != Some('(') {
          return false;
        }
      }
      '}' => {
        if stack.pop() != Some('{') {
          return false;
        }
      }
      ']' => {
        if stack.pop() != Some('[') {
          return false;
        }
      }
      _ => continue,
    }
  }
  stack.empty()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn bracket_matching_1() {
    let s = "(2+3){func}[abc]";
    assert!(brkt_match(s));
  }
  #[test]
  fn bracket_matching_2() {
    let s = "(2+3)*(3-1";
    assert!(!brkt_match(s));
  }
  #[test]
  fn bracket_matching_3() {
    let s = "{{([])}}";
    assert!(brkt_match(s));
  }
  #[test]
  fn bracket_matching_4() {
    let s = "{{(}[)]}";
    assert!(!brkt_match(s));
  }
  #[test]
  fn bracket_matching_5() {
    let s = "[[[]]]]]]]]]";
    assert!(!brkt_match(s));
  }
  #[test]
  fn bracket_matching_6() {
    let s = "";
    assert!(brkt_match(s));
  }
}
