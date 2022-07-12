impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
      let mut res = vec![vec![]];
      let mut seen = vec![false;nums.len()];
      for i in 0..nums.len() {
        Self::DSF(&nums, &mut vec![], i, &mut seen, &mut res);
      } 
      res     
    }
  
    pub fn DSF(nums: &Vec<i32>, tmp: &mut Vec<i32>,i: usize, seen: &mut Vec<bool>, res: &mut Vec<Vec<i32>>) {
        seen[i] = true;
        tmp.push(nums[i]);
        res.push(tmp.clone());
        for j in i..nums.len() {
          if seen[j] == false {
            Self::DSF(nums, tmp, j, seen, res);
          }
        }  
        tmp.pop();
        seen[i] = false;
    }
  }