impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let size = nums.len();
        let mut max = nums[0];
        let mut min = nums[size-1];
        let mut low =0;
        let mut high = size-1;
        for i in 0..size {
          if nums[i] >= max {
            max = nums[i];
          }
          else{
            low = i;
          }
          if(nums[size-i-1] <= min) {
            min = nums[size-i-1];
          }
          else{
            high = size-i-1;
          }
        }
        match low > high {
          true => (low-high+1) as i32,
          false => 0
        }
    }
  }