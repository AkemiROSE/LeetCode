impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut res = nums.len() as i32 +1;
        let mut sum = 0;
        for right in 0..nums.len(){
            sum += nums[right];
            while sum >= target {
              res = std::cmp::min(res, right as i32-left+1);
              sum -= nums[left as usize];
              left += 1;
            }
        }
        return match res > nums.len() as i32 {
          true => 0,
          false => res
        }
    }
  }