
impl Solution {
    pub fn check_possibility(mut nums: Vec<i32>) -> bool {
      if nums.len() == 1 {return true;}
      let mut count = 0;
      for i in 1..nums.len() {
        if nums[i] < nums[i-1]{
          count+=1;
          if i >= 2 && nums[i-2] > nums[i] {
            nums[i]= nums[i-1];
          }
          else{
            nums[i-1] = nums[i];
          }
        }
         
      }
      count <= 1
    }
  }