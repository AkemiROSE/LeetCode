impl Solution {
    pub fn calculate(s: String) -> i32 {
    let mut nums = vec![];
    let mut tmp = 0;
    let mut sign = b'+'; 
    let bytes = s.into_bytes();
    for i in 0..bytes.len() {
      if bytes[i].is_ascii_digit() {
        tmp = tmp * 10 + (bytes[i]-b'0') as i32;
      }
      if !bytes[i].is_ascii_digit() && bytes[i] != b' ' || i == bytes.len()-1{
         let cur_sign = bytes[i];
         match sign {
          b'+' => {nums.push(tmp);},
          b'-' => {nums.push(-tmp);},
          b'*' => {
            let num = nums.pop().unwrap();
            nums.push(tmp * num);
          },
          b'/' => {
            let num = nums.pop().unwrap();
            nums.push(num/tmp);
          },
          _ => {} 
        }
        sign = cur_sign;
        tmp = 0;
      }
    }     
    let mut res = 0;
    for num in nums {res += num;}
    res
  }
}