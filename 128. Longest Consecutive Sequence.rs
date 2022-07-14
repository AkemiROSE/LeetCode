impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: std::collections::HashSet<_> = nums.iter().collect();
        set.iter()
            .filter(|&&x| !set.contains(&(x - 1))) 
            .map(|&&x| (x..).take_while(|x| set.contains(x)).count())
            .max()
            .unwrap_or(0) as _
    }
}