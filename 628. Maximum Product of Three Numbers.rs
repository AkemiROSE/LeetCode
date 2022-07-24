impl Solution {
    pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 3 {return nums.iter().product();}
        nums.sort();
        let t1 = nums[len-3..].iter().product();
        let t2 = nums[0] * nums[1]*nums[len-1];
        match t1 > t2 {
          true => t1,
          false => t2
        }
        
    }
  }