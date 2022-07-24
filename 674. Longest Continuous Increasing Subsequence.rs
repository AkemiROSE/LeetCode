impl Solution {
  pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {return 1;}
    let mut left =  0;
    let mut right = 0;
    let mut res = 0;
    for i in 1..nums.len() {
      if nums[i] > nums[i-1] {right=i as i32;}
      else{
        res = std::cmp::max(res, right-left+1);
        left = i as i32;
      }
    }
    std::cmp::max(res, right-left+1)
  }
}