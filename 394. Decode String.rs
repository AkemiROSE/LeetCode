impl Solution {
  pub fn decode_string(s: String) -> String {
    let bytes = s.into_bytes();
    let mut strs = vec![];
    let mut nums = vec![];
    let mut i = 0;
    loop {
      if bytes[i].is_ascii_digit() {
        let mut num = 0;
        while  i < bytes.len() && bytes[i].is_ascii_digit() {
          num = num* 10 + (bytes[i] - b'0') as usize;
          i+=1;
        }
        nums.push(num);
      }
      else if bytes[i].is_ascii_alphabetic() {
        let mut s = String::new();
        while i < bytes.len() && bytes[i].is_ascii_alphabetic() {
          s.push(bytes[i] as char);
          i+=1;
       }
        strs.push(s);
      }
      else if bytes[i] == b']' {
        i+=1;
        let mut tmp = vec![];
        while let Some(s) = strs.pop() {
         if s.eq(&"["){break}
         tmp.insert(0, s); 
        }
        let num = nums.pop().unwrap();
        let mut ss = vec![];
        for _ in 0..num {
          ss.extend_from_slice(&tmp);
        }
        strs.push(ss.join(""));
      }
      else{
        strs.push("[".to_owned());
        i+=1;
      }
      if i >= bytes.len(){break;}
    }
    strs.join("")
  }
}