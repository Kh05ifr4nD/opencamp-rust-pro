use rand::Rng;
use std::cmp::max;

pub fn find_max_prime_fac(mut n: u128) -> u128 {
  let mut max_fac = 1;

  if n & 1 == 0 {
    max_fac = 2;
    n /= 2;
  }

  while n & 1 == 0 {
    n /= 2;
  }

  if n == 1 {
    return max_fac;
  }

  let mut ls_fac = vec![n];
  while let Some(x) = ls_fac.pop() {
    if x <= max_fac || x < 2 {
      continue;
    }

    if test_prime(x) {
      max_fac = max(max_fac, x);
      continue;
    }

    let p = pollard_rho(x);
    ls_fac.push(p);
    ls_fac.push(x / p);
  }

  max_fac
}

const fn gcd(a: u128, b: u128) -> u128 {
  const fn gcd_inner(a: u128, b: u128) -> u128 {
    if b == 0 {
      a
    } else {
      gcd_inner(b, a % b)
    }
  }
  gcd_inner(a, b)
}

pub const fn mul_mod(a: u128, b: u128, m: u128) -> u128 {
  let mut res = 0;
  let mut a = a % m;
  let mut b = b % m;

  while b > 0 {
    if b & 1 == 1 {
      res = (res + a) % m;
      if res >= m {
        res -= m;
      }
    }
    a = (a + a) % m;
    if a >= m {
      a -= m;
    }
    b >>= 1;
  }
  res
}

const fn qpow(mut x: u128, mut p: u128, m: u128) -> u128 {
  let mut ans = 1;
  x %= m;
  while p > 0 {
    if p & 1 == 1 {
      ans = mul_mod(ans, x, m);
    }
    x = mul_mod(x, x, m);
    p >>= 1;
  }
  ans % m
}

fn test_prime(n: u128) -> bool {
  if n < 2 {
    return false;
  }
  if n == 2 || n == 3 {
    return true;
  }
  if n & 1 == 0 || n % 3 == 0 {
    return false;
  }

  // 将n-1分解为d*2^r的形式
  let mut d = n - 1;
  let mut r = 0;
  while d % 2 == 0 {
    d /= 2;
    r += 1;
  }

  // 对128位整数,这些基数已经足够判定素性
  let ls_base = [2, 325, 9375, 28178, 450_775, 9_780_504, 1_795_265_022];

  'witness: for &a in &ls_base {
    if a >= n {
      continue;
    }

    let mut x = qpow(a, d, n);
    if x == 1 || x == n - 1 {
      continue;
    }

    for _ in 1..r {
      x = mul_mod(x, x, n);
      if x == n - 1 {
        continue 'witness;
      }
      if x == 1 {
        return false;
      }
    }
    return false;
  }

  true
}

fn pollard_rho(n: u128) -> u128 {
  if n % 2 == 0 {
    return 2;
  }
  if n % 3 == 0 {
    return 3;
  }

  let mut rng = rand::rng();
  let mut x = rng.random_range(2..n);
  let mut y = x;
  let mut c = rng.random_range(1..n);
  let mut d = 1;

  while d == 1 {
    // 龟步:x = f(x)
    x = (mul_mod(x, x, n) + c) % n;

    // 兔步:y = f(f(y))
    y = (mul_mod(y, y, n) + c) % n;
    y = (mul_mod(y, y, n) + c) % n;

    let diff = if x >= y { x - y } else { y - x };
    d = gcd(diff, n);

    if d > 1 && d < n {
      return d;
    }

    if d == n {
      x = rng.random_range(2..n);
      y = x;
      c = rng.random_range(1..n);
      d = 1;
    }
  }

  d
}
