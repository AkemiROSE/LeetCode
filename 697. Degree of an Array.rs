use std::collections::HashMap;
impl Solution {
  pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
      let mut mp = HashMap::new();
      for i in 0..nums.len() {
        if mp.get(&nums[i]).is_none() {
          mp.insert(nums[i], vec![i]);
        }
        else{
          mp.get_mut(&nums[i]).unwrap().push(i);
        }
      }
      if mp.len() == nums.len() {return 1;}
      let mut times = 0;let mut res = nums.len();
      for (num, indexs) in mp.into_iter() {
        if indexs.len() > times {        
            res = indexs[indexs.len()-1]-indexs[0]+1;
            times = indexs.len();
        }
        else if indexs.len() == times{
          res = std::cmp::min(res, indexs[indexs.len()-1]-indexs[0]+1);
        }                    
    }
      res as _
  }
}