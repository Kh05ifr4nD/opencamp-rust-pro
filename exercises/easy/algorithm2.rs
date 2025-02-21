/*
  double linked list reverse
  This problem requires you to reverse a doubly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
  val: T,
  next: Option<NonNull<Node<T>>>,
  prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
  fn new(t: T) -> Node<T> {
    Node { val: t, prev: None, next: None }
  }
}
#[derive(Debug)]
struct LinkedLs<T> {
  len: usize,
  bgn: Option<NonNull<Node<T>>>,
  end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedLs<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T> LinkedLs<T> {
  pub fn new() -> Self {
    Self { len: 0, bgn: None, end: None }
  }

  pub fn append(&mut self, obj: T) {
    let mut node = Box::new(Node::new(obj));
    node.next = None;
    node.prev = self.end;
    let ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
    match self.end {
      None => self.bgn = ptr,
      Some(ptr_end) => unsafe { (*ptr_end.as_ptr()).next = ptr },
    }
    self.end = ptr;
    self.len += 1;
  }

  pub fn get(&mut self, idx: usize) -> Option<&T> {
    let mut node_cur = self.bgn;
    let mut idx = idx;
    while let Some(ptr_cur) = node_cur {
      if idx == 0 {
        return Some(unsafe { &(*ptr_cur.as_ptr()).val });
      }
      idx -= 1;
      node_cur = unsafe { (*ptr_cur.as_ptr()).next };
    }
    None
  }

  pub fn rev_ip(&mut self) {
    match self.len {
      0 | 1 => (),
      _ => {
        let mut node_prev = None::<NonNull<Node<T>>>;
        let mut node_cur = self.bgn;
        unsafe {
          for _ in 0..self.len {
            let ptr_cur = node_cur.unwrap_unchecked().as_ptr();
            let node_next = (*ptr_cur).next;
            (*ptr_cur).next = node_prev;
            (*ptr_cur).prev = node_next;
            node_prev = node_cur;
            node_cur = node_next;
          }
          std::mem::swap(&mut self.end, &mut self.bgn);
        }
      }
    }
  }
}

impl<T> Display for LinkedLs<T>
where
  T: Display,
{
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self.bgn {
      Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
      None => Ok(()),
    }
  }
}

impl<T> Display for Node<T>
where
  T: Display,
{
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self.next {
      Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
      None => write!(f, "{}", self.val),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::LinkedLs;

  #[test]
  fn create_numeric_list() {
    let mut list = LinkedLs::<i32>::new();
    list.append(1);
    list.append(2);
    list.append(3);
    println!("Linked List is {list}");
    assert_eq!(3, list.len);
  }

  #[test]
  fn create_string_list() {
    let mut list_str = LinkedLs::<String>::new();
    list_str.append("A".to_string());
    list_str.append("B".to_string());
    list_str.append("C".to_string());
    println!("Linked List is {list_str}");
    assert_eq!(3, list_str.len);
  }

  #[test]
  fn test_reverse_linked_list_1() {
    let mut list = LinkedLs::<i32>::new();
    let original_vec = [2, 3, 5, 11, 9, 7];
    let reverse_vec = [7, 9, 11, 5, 3, 2];
    for elem in original_vec {
      list.append(elem);
    }
    println!("Linked List is {list}");
    list.rev_ip();
    println!("Reversed Linked List is {list}");
    for (i, &elem) in reverse_vec.iter().enumerate() {
      assert_eq!(elem, *list.get(i).unwrap());
    }
  }

  #[test]
  fn test_reverse_linked_list_2() {
    let mut list = LinkedLs::<i32>::new();
    let original_vec = [34, 56, 78, 25, 90, 10, 19, 34, 21, 45];
    let reverse_vec = [45, 21, 34, 19, 10, 90, 25, 78, 56, 34];
    for elem in original_vec {
      list.append(elem);
    }
    println!("Linked List is {list}");
    list.rev_ip();
    println!("Reversed Linked List is {list}");
    for (i, &elem) in reverse_vec.iter().enumerate() {
      assert_eq!(elem, *list.get(i).unwrap());
    }
  }
}
