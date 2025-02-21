pub fn bday_prob(n: u32) -> f64 {
  if n == 0 {
    return 0.0;
  }
  if n >= 365 {
    return 1.0;
  }

  let mut prob = 1.0;
  for i in 0..n {
    prob *= (365.0 - i as f64) / 365.0;
  }

  1.0 - prob
}
