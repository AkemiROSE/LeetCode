impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let row = matrix.len();
        let col = matrix[0].len();
        let mut dp = vec![vec![0;col+1];row+1];
        let mut res = 0;
        for i in 1..=row {
            for j in 1..=col {
                if matrix[i-1][j-1] == '1' {
                    dp[i][j] = std::cmp::min(std::cmp::min(dp[i-1][j-1],dp[i-1][j]),dp[i][j-1]) + 1;
                    res = std::cmp::max(res, dp[i][j]);
                }
            }
        }
        res*res as i32
    }
}