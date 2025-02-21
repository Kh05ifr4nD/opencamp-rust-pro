/*
    Find Intersection of Two Arrays
    Given two arrays, find the intersection of the arrays and return the elements of the intersection (without duplicates).
    The result should not contain any duplicate elements.

    You need to implement the function `intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32>`.
    The function should return a vector containing all the elements that are in both arrays.

    Hint: You can solve this problem using sorting, hash sets, or the two-pointer technique.
*/

use std::collections::HashSet;

pub fn intxn(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
  let set1: HashSet<_> = nums1.into_iter().collect();
  let set2: HashSet<_> = nums2.into_iter().collect();
  let mut rslt: Vec<i32> = set1.intersection(&set2).copied().collect();
  rslt.sort_unstable();
  rslt
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_intersection_1() {
    let nums1 = vec![1, 2, 2, 1];
    let nums2 = vec![2, 2];
    let result = intxn(nums1, nums2);
    println!("Intersection: {:?}", result);
    assert_eq!(result, vec![2]);
  }

  #[test]
  fn test_intersection_2() {
    let nums1 = vec![4, 9, 5];
    let nums2 = vec![9, 4, 9, 8, 4];
    let result = intxn(nums1, nums2);
    println!("Intersection: {:?}", result);
    assert_eq!(result, vec![4, 9]);
  }

  #[test]
  fn test_intersection_3() {
    let nums1 = vec![1, 2, 3];
    let nums2 = vec![4, 5, 6];
    let result = intxn(nums1, nums2);
    println!("Intersection: {:?}", result);
    assert_eq!(result, Vec::<i32>::new());
  }

  #[test]
  fn test_intersection_4() {
    let nums1 = vec![1, 1, 1];
    let nums2 = vec![1, 1, 1];
    let result = intxn(nums1, nums2);
    println!("Intersection: {:?}", result);
    assert_eq!(result, vec![1]);
  }

  #[test]
  fn test_intersection_5() {
    let nums1 = vec![10, 20, 30];
    let nums2 = vec![30, 40, 50];
    let result = intxn(nums1, nums2);
    println!("Intersection: {:?}", result);
    assert_eq!(result, vec![30]);
  }
}
