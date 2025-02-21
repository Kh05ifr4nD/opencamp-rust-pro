/*
  sort
  This problem requires you to implement a sorting algorithm
  you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: PartialOrd>(arr: &mut [T]) {
  fn heapify<T: PartialOrd>(arr: &mut [T], mut root: usize, end: usize) {
    loop {
      let l = 2 * root + 1;
      if l >= end {
        break;
      }
      let r = l + 1;
      let mut lgest = root;
      if arr[l] > arr[lgest] {
        lgest = l;
      }
      if r < end && arr[r] > arr[lgest] {
        lgest = r;
      }

      if lgest == root {
        break;
      }

      arr.swap(root, lgest);
      root = lgest;
    }
  }

  let len = arr.len();
  for i in (0..len / 2).rev() {
    heapify(arr, i, len);
  }

  for end in (1..len).rev() {
    arr.swap(0, end);
    heapify(arr, 0, end);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_sort_1() {
    let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
    sort(&mut vec);
    assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
  }
  #[test]
  fn test_sort_2() {
    let mut vec = vec![1];
    sort(&mut vec);
    assert_eq!(vec, vec![1]);
  }
  #[test]
  fn test_sort_3() {
    let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
    sort(&mut vec);
    assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
  }
}
