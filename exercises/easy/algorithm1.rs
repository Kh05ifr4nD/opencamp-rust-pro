/*
  single linked list merge
  This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
  val: T,
  next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
  fn new(val: T) -> Node<T> {
    Node { val, next: None }
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
    let node = Some(unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(Node::new(obj)))) });
    if let Some(ptr_end) = self.end {
      unsafe { (*ptr_end.as_ptr()).next = node }
    } else {
      self.bgn = node;
    }
    self.end = node;
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
}

impl<T: PartialOrd> LinkedLs<T> {
  pub fn merge(ls_a: &LinkedLs<T>, ls_b: &LinkedLs<T>) -> Self {
    let mut c = LinkedLs::new();
    let (mut node_a, mut node_b) = (ls_a.bgn, ls_b.bgn);

    while let (Some(ptr_a), Some(ptr_b)) = (node_a, node_b) {
      unsafe {
        let (node_next_a, node_next_b) = ((*ptr_a.as_ptr()).next, (*ptr_b.as_ptr()).next);

        if (*ptr_a.as_ptr()).val <= (*ptr_b.as_ptr()).val {
          (*ptr_a.as_ptr()).next = None;
          c.append_node(ptr_a);
          node_a = node_next_a;
        } else {
          (*ptr_b.as_ptr()).next = None;
          c.append_node(ptr_b);
          node_b = node_next_b;
        }
      }
    }

    let mut rem = node_a.or(node_b);
    while let Some(node) = rem {
      unsafe {
        let next = (*node.as_ptr()).next;
        (*node.as_ptr()).next = None;
        c.append_node(node);
        rem = next;
      }
    }
    c
  }

  fn append_node(&mut self, node: NonNull<Node<T>>) {
    match self.end {
      None => {
        self.bgn = Some(node);
        self.end = Some(node);
      }
      Some(ptr_end) => unsafe {
        (*ptr_end.as_ptr()).next = Some(node);
        self.end = Some(node);
      },
    }
    self.len += 1;
  }
}

impl<T> Display for LinkedLs<T>
where
  T: Display,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
  fn test_merge_linked_list_1() {
    let mut list_a = LinkedLs::<i32>::new();
    let mut list_b = LinkedLs::<i32>::new();
    let vec_a = [1, 3, 5, 7];
    let vec_b = [2, 4, 6, 8];
    let target_vec: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

    for i in vec_a {
      list_a.append(i);
    }
    for i in vec_b {
      list_b.append(i);
    }
    println!("list a {list_a} list b {list_b}");
    let mut list_c = LinkedLs::<i32>::merge(&list_a, &list_b);
    println!("merged List is {list_c}");
    for i in target_vec {
      assert_eq!(i, *list_c.get(i as usize - 1).unwrap());
    }
  }
  #[test]
  fn test_merge_linked_list_2() {
    let mut list_a = LinkedLs::<i32>::new();
    let mut list_b = LinkedLs::<i32>::new();
    let vec_a = [11, 33, 44, 88, 89, 90, 100];
    let vec_b = [1, 22, 30, 45];
    let target_vec = [1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

    for i in vec_a {
      list_a.append(i);
    }
    for i in vec_b {
      list_b.append(i);
    }
    println!("list a {list_a} list b {list_b}");
    let mut list_c = LinkedLs::<i32>::merge(&list_a, &list_b);
    println!("merged List is {list_c}");
    for (i, &item) in target_vec.iter().enumerate() {
      assert_eq!(*list_c.get(i).unwrap(), item);
    }
  }
}
