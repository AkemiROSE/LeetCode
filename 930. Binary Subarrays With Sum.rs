use std::collections::HashMap;
impl Solution {
  pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut mp = HashMap::new();
        let mut count = 0;
        let mut presum = 0;
        mp.insert(0, 1);
        for num in nums {
          presum += num;          
          let key = presum - goal;
          count +=  *mp.entry(key).or_default();
          *mp.entry(presum).or_default() += 1
        }
        count
  }
}