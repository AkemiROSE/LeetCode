impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
      let mut res = nums[0];
      let mut imax= res;
      let mut imin = res;
      for i in 1..nums.len() {
        if nums[i] < 0 {std::mem::swap(&mut imax, &mut imin);}
        imax = std::cmp::max(nums[i], nums[i]*imax);
        imin = std::cmp::min(nums[i], nums[i]*imin);
        res = std::cmp::max(res, imax);
      }
      res
    }
  }