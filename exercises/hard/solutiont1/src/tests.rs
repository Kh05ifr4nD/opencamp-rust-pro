// src/tests.rs
mod conjecture;

#[cfg(test)]
mod tests {
  use super::conjecture::goldbach_conjecture;
  use std::time::{Duration, Instant};

  // 定义测试用例和预期结果
  const TEST_CASE: &str = "5777,5993";

  // 定义一个测试函数来验证每个测试用例
  #[test]
  fn test_conjecture() {
    let start = Instant::now();
    let result = goldbach_conjecture();
    let duration = start.elapsed();

    // 时间超0.5s，判定不合格
    let mut total_score = 0.0;
    if duration <= Duration::from_millis(200) && result == TEST_CASE {
      total_score += 100.0;
    }

    println!("Total score: {:.2}", total_score);
    assert_eq!(100.00, total_score);
  }
}
