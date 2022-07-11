impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
      let mut dp = vec![vec![0;1001]; 1001];
      for (i,ch1) in text1.chars().enumerate() {
        for (j, ch2) in text2.chars().enumerate() {
          dp[i+1][j+1] = match ch1.eq(&ch2) {
            true => {dp[i][j]+1},
            false =>{std::cmp::max(dp[i][j+1], dp[i+1][j])}
          };
        }
      }
      dp[text1.len()][text2.len()]
    }
  }