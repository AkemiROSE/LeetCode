use std::collections::HashMap;
impl Solution {
  pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
      let mut mp = HashMap::new();
      mp.insert(0,-1);
      let mut presum = 0;
      for (i, num) in nums.into_iter().enumerate() {
        presum += num;
        let key = match k == 0 {
          true => {presum},
          false => {
            presum % k
          }
        };
        if mp.contains_key(&key) {
          if i as i32 - *mp.get(&key).unwrap() >= 2 {
            return true;
          }
          continue;
        }
        mp.insert(key, i as i32);
      } 
      false
  }
}