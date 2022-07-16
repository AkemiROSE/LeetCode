impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
      let mut numstr:Vec<_> = nums.iter().map(|n| n.to_string()).collect();
      numstr.sort_by(|a,b|{
        format!("{}{}",a,b).cmp(&format!("{}{}",b,a))
      });
      numstr.reverse();
      let res: String = numstr.join("");
      if let Some(pos) = res.chars().position(|c| c!= '0'){
        return res[pos..].to_owned();
      }
      else{
        return "0".to_owned();
      }
  
    }
  }