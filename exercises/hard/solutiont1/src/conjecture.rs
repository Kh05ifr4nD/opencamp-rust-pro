pub fn goldbach_conjecture() -> String {
  const MAX_N: usize = 6000;
  let mut sieve = vec![true; MAX_N + 1];
  sieve[0] = false;
  sieve[1] = false;
  for i in 2..=MAX_N.isqrt() {
    if sieve[i] {
      for j in (i * i..=MAX_N).step_by(i) {
        sieve[j] = false;
      }
    }
  }

  let is_prime = |n: usize| -> bool {
    if n > MAX_N {
      if n <= 1 {
        return false;
      }
      if n <= 3 {
        return true;
      }
      if n % 2 == 0 || n % 3 == 0 {
        return false;
      }
      let mut i = 5;
      while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
          return false;
        }
        i += 6;
      }
      true
    } else {
      sieve[n as usize]
    }
  };

  let mut ls_rslt = Vec::with_capacity(2);
  let mut n = 9;

  while ls_rslt.len() < 2 {
    if n % 2 == 1 && !is_prime(n) {
      let mut found = false;
      let mut k = 1;

      while 2 * k * k < n {
        let p = n - 2 * k * k;
        if is_prime(p) {
          found = true;
          break;
        }
        k += 1;
      }

      if !found {
        ls_rslt.push(n);
      }
    }
    n += 2;
  }

  format!("{},{}", ls_rslt[0], ls_rslt[1])
}
