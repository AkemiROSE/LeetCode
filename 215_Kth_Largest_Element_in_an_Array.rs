use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut queue = BinaryHeap::with_capacity(k);
        for &num in nums.iter() {
            if queue.len() < k {
                queue.push(Reverse(num));
            }
            else if num >= queue.peek().unwrap().0{
                queue.pop();
                queue.push(Reverse(num));
            }
        }
        queue.peek().unwrap().0
    }
}