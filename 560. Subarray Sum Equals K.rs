use std::collections::HashMap;
impl Solution {
  pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    if nums.len() == 0 {return 0;}  
    let mut mp = HashMap::new();
    mp.insert(0, 1);
    let mut count = 0;
    let mut presum = 0;
    for num in nums {
      presum += num;
      let tmp = presum-k;
      if mp.contains_key(&tmp){count+=*mp.get(&tmp).unwrap();}
      *mp.entry(presum).or_insert(0) += 1;
    }
    count
  }
}