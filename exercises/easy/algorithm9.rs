/*
  heap
  Binary heap implementation with separated MinHeap/MaxHeap types
*/

use std::cmp::Ord;
use std::default::Default;

pub trait HeapBeh<T: Ord> {
  fn cmp(a: &T, b: &T) -> bool;
}

pub struct MinHeapBeh;
impl<T: Ord> HeapBeh<T> for MinHeapBeh {
  fn cmp(a: &T, b: &T) -> bool {
    a < b
  }
}

pub struct MaxHeapBeh;
impl<T: Ord> HeapBeh<T> for MaxHeapBeh {
  fn cmp(a: &T, b: &T) -> bool {
    a > b
  }
}

pub struct Heap<T: Default + Ord, B: HeapBeh<T>> {
  size: usize,
  ctnr: Vec<T>,
  _mkr: std::marker::PhantomData<B>,
}

impl<T, B> Heap<T, B>
where
  T: Default + Ord,
  B: HeapBeh<T>,
{
  pub fn new() -> Self {
    Self { size: 0, ctnr: vec![T::default()], _mkr: std::marker::PhantomData }
  }

  pub const fn size(&self) -> usize {
    self.size
  }

  pub const fn empty(&self) -> bool {
    0 == self.size
  }

  const fn exist_child(&self, idx: usize) -> bool {
    idx * 2 <= self.size
  }

  fn cmp_node(a: &T, b: &T) -> bool {
    B::cmp(a, b)
  }

  fn smallest_child_idx(&self, idx: usize) -> usize {
    let left = idx * 2;
    let right = left + 1;
    if right > self.size || Self::cmp_node(&self.ctnr[left], &self.ctnr[right]) {
      left
    } else {
      right
    }
  }

  pub fn add(&mut self, val: T) {
    self.ctnr.push(val);
    self.size += 1;
    let mut idx = self.size;
    while idx > 1 {
      let idx_prnt = idx / 2;
      if Self::cmp_node(&self.ctnr[idx], &self.ctnr[idx_prnt]) {
        self.ctnr.swap(idx, idx_prnt);
        idx = idx_prnt;
      } else {
        break;
      }
    }
  }
}

pub type MinHeap<T> = Heap<T, MinHeapBeh>;
pub type MaxHeap<T> = Heap<T, MaxHeapBeh>;

impl<T, B> Iterator for Heap<T, B>
where
  T: Default + Ord,
  B: HeapBeh<T>,
{
  type Item = T;

  fn next(&mut self) -> Option<T> {
    if self.size == 0 {
      return None;
    }
    let root = self.ctnr.swap_remove(1);
    self.size -= 1;
    if self.size > 0 {
      let mut idx = 1;
      while self.exist_child(idx) {
        let child = self.smallest_child_idx(idx);
        if Self::cmp_node(&self.ctnr[child], &self.ctnr[idx]) {
          self.ctnr.swap(idx, child);
          idx = child;
        } else {
          break;
        }
      }
    }
    Some(root)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_empty_heap() {
    let mut heap = MaxHeap::<i32>::new();
    assert_eq!(heap.next(), None);
  }

  #[test]
  fn test_min_heap() {
    let mut heap = MinHeap::new();
    heap.add(4);
    heap.add(2);
    heap.add(9);
    heap.add(11);
    assert_eq!(heap.size(), 4);
    assert_eq!(heap.next(), Some(2));
    assert_eq!(heap.next(), Some(4));
    assert_eq!(heap.next(), Some(9));
    heap.add(1);
    assert_eq!(heap.next(), Some(1));
  }

  #[test]
  fn test_max_heap() {
    let mut heap = MaxHeap::new();
    heap.add(4);
    heap.add(2);
    heap.add(9);
    heap.add(11);
    assert_eq!(heap.size(), 4);
    assert_eq!(heap.next(), Some(11));
    assert_eq!(heap.next(), Some(9));
    assert_eq!(heap.next(), Some(4));
    heap.add(1);
    assert_eq!(heap.next(), Some(2));
  }
}
