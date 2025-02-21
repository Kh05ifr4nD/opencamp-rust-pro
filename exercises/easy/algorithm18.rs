/*
    Merge Intervals
    Given an array of intervals where each interval is represented by a pair of integers [start, end],
    merge all overlapping intervals and return a list of non-overlapping intervals.

    The intervals are inclusive, meaning the interval [start, end] includes both start and end points.

    You need to implement the function `merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.
    The function should return a vector containing all the merged intervals.

    Hint: You can start by sorting the intervals by their starting point and then merge them one by one.
*/

pub fn merge_intvl(mut ls_intvl: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
  if ls_intvl.is_empty() {
    return Vec::new();
  }

  ls_intvl.sort_by_key(|v| v[0]);

  let mut rslt = vec![ls_intvl[0].clone()];

  for intvl in ls_intvl.into_iter().skip(1) {
    let last = rslt.last_mut().unwrap();

    if intvl[0] <= last[1] {
      last[1] = last[1].max(intvl[1]);
    } else {
      rslt.push(intvl);
    }
  }

  rslt
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_merge_intervals_1() {
    let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
    let result = merge_intvl(intervals);
    println!("Merged intervals: {result:?}");
    assert_eq!(result, vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
  }

  #[test]
  fn test_merge_intervals_2() {
    let intervals = vec![vec![1, 4], vec![4, 5]];
    let result = merge_intvl(intervals);
    println!("Merged intervals: {result:?}");
    assert_eq!(result, vec![vec![1, 5]]);
  }

  #[test]
  fn test_merge_intervals_3() {
    let intervals = vec![vec![1, 4], vec![0, 4]];
    let result = merge_intvl(intervals);
    println!("Merged intervals: {result:?}");
    assert_eq!(result, vec![vec![0, 4]]);
  }

  #[test]
  fn test_merge_intervals_4() {
    let intervals = vec![vec![1, 10], vec![2, 6], vec![8, 10]];
    let result = merge_intvl(intervals);
    println!("Merged intervals: {result:?}");
    assert_eq!(result, vec![vec![1, 10]]);
  }

  #[test]
  fn test_merge_intervals_5() {
    let intervals = vec![vec![1, 2], vec![3, 5], vec![4, 7], vec![8, 10]];
    let result = merge_intvl(intervals);
    println!("Merged intervals: {result:?}");
    assert_eq!(result, vec![vec![1, 2], vec![3, 7], vec![8, 10]]);
  }
}
