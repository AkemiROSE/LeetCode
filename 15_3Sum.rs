impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
     if nums.len() < 3 {return vec![];}
     let mut results = vec![];
     let mut nums = nums;
     let mut i = 0;
     nums.sort();
     while i < (nums.len()-2) {
       let num1 = nums[i];
       let mut j= i+1;
       let mut k = nums.len() - 1;
       while j < k {
         let sum = num1 + nums[j] + nums[k];
         if sum == 0 {
           results.push(vec![num1, nums[j], nums[k]]);
           j += 1;
           while j < k && nums[j] == nums[j-1] {j +=1;}
           k -= 1;
           while j < k && nums[k] == nums[k+1] {k -= 1}
         }
         else if sum > 0 {
           k -= 1;
           while j < k && nums[k] == nums[k+1] {k -= 1}
         }
         else{
           j += 1;
           while j < k && nums[j] == nums[j-1] {j +=1;}
         }
       }
       
       i += 1;
       while i < (nums.len()-2) && nums[i] == nums[i-1] {i += 1}
     }
 
     results
   }
 }