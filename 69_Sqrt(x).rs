impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {return 0;}
        let mut left = 1;
        let mut right =  x/2;
        loop {
            let mid = left + (right - left)/2;
            if mid > x / mid {
                right = mid - 1;
            }
            else{
                if (mid+1) > x/(mid+1) {
                    return mid;
                }
                else{
                    left = mid + 1;
                }
            }
        }
    }
}