pub fn dp_rec_mc(amount: u32) -> u32 {
  if amount == 0 {
    return 0;
  }

  let coins = [100usize, 50, 30, 20, 10, 5, 2, 1];

  let mut dp = vec![u32::MAX; (amount + 1) as usize];
  dp[0] = 0;
  for i in 1..=amount as usize {
    for coin in coins {
      if coin <= i && dp[i - coin] != u32::MAX {
        dp[i] = dp[i].min(dp[i - coin] + 1);
      }
    }
  }

  if dp[amount as usize] == u32::MAX {
    return 0;
  }

  dp[amount as usize]
}
