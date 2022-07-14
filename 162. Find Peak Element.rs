impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let mid1 = left + ((right - left)>>2);
            let mid2 = mid1+1;
            if nums[mid1] < nums[mid2] {
                left = mid2;
            }
            else{
                right = mid1;
            }
        }
        left as i32
    }
}