impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
      let mut res = vec![];
      for i1 in 0..3 {
        for i2 in 0..3 {
            let p2 = i1+1;
            if p2 >= s.len() {break;}
          for i3 in 0..3 {       
              let p3 = p2+i2+1;
              let p4 = p3+i3+1;
              if p3 >= s.len() || p4 >= s.len() {break;}
              let mut flag = true;
              let tmp = vec![&s[0..=i1],&s[p2..=p2+i2],&s[p3..=p3+i3],&s[p4..]];
              for &num_str in &tmp {
                if !Self::valid(num_str) {
                  flag = false;
                  break;
                }
              } 
              if flag {
                res.push(tmp.join("."));
              }
          }
        }
      }
      res
    }
    fn valid(num_str:&str) -> bool {
      let mut flag = false;
      if num_str.starts_with("0") {flag = true;}
      let num = num_str.parse::<usize>().unwrap();
      if num == 0 && num_str.eq("0") {return true;}
      if num >0 && num <= 255 && !flag {
        return true;
      }
      false
    }
  }