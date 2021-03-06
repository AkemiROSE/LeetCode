impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
      let total_sum: i32 = nums.iter().sum();
      let mut left_sum = 0;
      for i in 0.. nums.len() {
        if left_sum == total_sum - nums[i] - left_sum {
          return i as i32;
        }
        left_sum += nums[i];
      }
      -1
    }
  }