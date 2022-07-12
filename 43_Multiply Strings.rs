impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {return "0".to_owned();}
        if num1 == "1" {return num2;}
        if num2 == "1"{return num1;}
        let nums1 = num1.into_bytes();
        let nums2 = num2.into_bytes();
        let mut res = vec![b'0'; nums1.len()+nums2.len()];
        for i in (0..nums1.len()).rev() {
          let p1 = nums1[i] as u8 - b'0';
          for j in (0..nums2.len()).rev() {
            let p2 = nums2[j] as u8 - b'0';
            let tmp = p1 * p2 + res[i+j+1] -b'0';
            res[i+j+1] = tmp % 10 +b'0';
            res[i+j] += tmp / 10;
          }
        }
        let mut s = 0;
        for i in 0..res.len() {
          if res[i] != b'0' {
            s = i;
            break;
          }
        }
        std::str::from_utf8(&res[s..]).unwrap().to_owned()
  }
}