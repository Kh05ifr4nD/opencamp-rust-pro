/*
  binary_search tree
  This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
  T: Ord,
{
  val: T,
  l: Option<Box<TreeNode<T>>>,
  r: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinSrchTree<T>
where
  T: Ord,
{
  root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
  T: Ord,
{
  fn new(value: T) -> Self {
    TreeNode { val: value, l: None, r: None }
  }
}

impl<T> BinSrchTree<T>
where
  T: Ord,
{
  fn new() -> Self {
    BinSrchTree { root: None }
  }

  fn insert(&mut self, value: T) {
    if let Some(root) = &mut self.root {
      root.insert(value);
    } else {
      self.root = Some(Box::new(TreeNode::new(value)));
    }
  }

  fn search(&self, value: T) -> bool {
    self.root.as_ref().is_some_and(|root| root.search(&value))
  }
}

impl<T> TreeNode<T>
where
  T: Ord,
{
  fn insert(&mut self, value: T) {
    match value.cmp(&self.val) {
      Ordering::Less => {
        if let Some(left) = &mut self.l {
          left.insert(value);
        } else {
          self.l = Some(Box::new(TreeNode::new(value)));
        }
      }
      Ordering::Greater => {
        if let Some(right) = &mut self.r {
          right.insert(value);
        } else {
          self.r = Some(Box::new(TreeNode::new(value)));
        }
      }
      Ordering::Equal => {}
    }
  }

  fn search(&self, value: &T) -> bool {
    match value.cmp(&self.val) {
      Ordering::Equal => true,
      Ordering::Less => self.l.as_ref().is_some_and(|left| left.search(value)),
      Ordering::Greater => self.r.as_ref().is_some_and(|right| right.search(value)),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_insert_and_search() {
    let mut bst = BinSrchTree::new();

    assert!(!bst.search(1));

    bst.insert(5);
    bst.insert(3);
    bst.insert(7);
    bst.insert(2);
    bst.insert(4);

    assert!(bst.search(5));
    assert!(bst.search(3));
    assert!(bst.search(7));
    assert!(bst.search(2));
    assert!(bst.search(4));

    assert!(!bst.search(1));
    assert!(!bst.search(6));
  }

  #[test]
  fn test_insert_duplicate() {
    let mut bst = BinSrchTree::new();

    bst.insert(1);
    bst.insert(1);

    assert!(bst.search(1));

    match bst.root {
      Some(ref node) => {
        assert!(node.l.is_none());
        assert!(node.r.is_none());
      }
      None => panic!("Root should not be None after insertion"),
    }
  }
}
