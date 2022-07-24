impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
      let mut presum = 0;
      for i in 0..k as usize {
        presum+=nums[i];
      } 
      let mut res = presum as f64 / k as f64;
      for i in k as usize..nums.len(){
        presum += nums[i];
        presum -= nums[i-k as usize];
        let tmp = presum as f64 / k as f64;
        if tmp > res {res = tmp;}
      }
      res
    }
  }