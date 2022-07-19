use std::collections::HashMap;
impl Solution {
  pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
      let mut mp = HashMap::new();
      mp.insert(0, 1);
      let mut count = 0;
      let mut presum = 0;
      for num in nums {
          presum += num;
          let key = (presum % k + k) % k;
          if mp.contains_key(&key) {
            count += *mp.get(&key).unwrap();
          }
          *mp.entry(key).or_default() += 1;
      } 
      count
  }
}