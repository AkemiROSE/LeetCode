impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
      let mut major = nums[0];
      let mut count = 0;
      for n in nums{
        if count == 0 {
          major = n;
          count = 1;
        }
        else if major == n {
          count += 1;
        }
        else{
          count -= 1;
        }
      }
      major
    }
  }