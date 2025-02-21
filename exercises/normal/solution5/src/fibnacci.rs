pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
  let mut sum = 0;
  let mut prev = 0;
  let mut curr = 1;

  while curr < threshold {
    if curr % 2 == 1 {
      sum += curr;
    }

    let next = prev + curr;
    prev = curr;
    curr = next;
  }

  sum
}
