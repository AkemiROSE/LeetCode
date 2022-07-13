impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
      let row = grid.len();
      let col = grid[0].len();
      let mut dp = vec![vec![0i32;col];row];
      dp[0][0] = grid[0][0];
      for i in 1..row{dp[i][0] = dp[i-1][0] + grid[i][0];}
      for j in 1..col{dp[0][j] = dp[0][j-1] + grid[0][j];}
      for i in 1..row {
        for j in 1..col {
          dp[i][j] = std::cmp::min(dp[i-1][j] ,dp[i][j-1]) + grid[i][j];
        }
      }
      dp[row-1][col-1]
    }
  }