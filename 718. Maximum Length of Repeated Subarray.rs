impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0;nums2.len()+1]; nums1.len()+1];
        let mut res = 0;
        for (i, n1) in nums1.iter().enumerate(){
            for (j, n2) in nums2.iter().enumerate(){
                if n1 == n2 {
                    dp[i+1][j+1] = dp[i][j]+1;
                    res = std::cmp::max(res, dp[i+1][j+1]);
                }
                else{
                    dp[i+1][j+1] = 0;
                }
            }
        }
        res
    }
}