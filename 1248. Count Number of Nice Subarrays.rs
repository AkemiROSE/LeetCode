impl Solution {
  pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut mp = vec![0;nums.len()+1];
        mp[0] = 1;
        let mut res = 0;
        let mut count = 0;
        for num in nums {
            count += num % 2;
            if count >= k {
              res += mp[(count-k) as usize]
            }
            mp[count as usize] += 1; 
        }
        res
  }
}