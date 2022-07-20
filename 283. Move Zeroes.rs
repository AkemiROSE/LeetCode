impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
      let mut i =0;
      let mut j = 0;
      loop {
        while i < nums.len() && nums[i] == 0 {i += 1;}
        while j < nums.len() && nums[j] != 0 {j += 1;}
        if i >= nums.len() ||j >= nums.len() {break;}
        if i > j {
          nums.swap(i, j);
        }
        else{
            i = j;
        }
      }
    }
  }