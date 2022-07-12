impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
      let mut dp = vec![amount+1; amount as usize+1];
      dp[0] = 0;
       for &coin in &coins {
        for i in coin..=amount {
          dp[i as usize] = std::cmp::min(dp[i as usize], dp[(i-coin) as usize]+1);
        }
       }
       if dp[amount as usize] > amount {
        return -1;
       } 
       else{
        return dp[amount as usize];
       }
    }
  }