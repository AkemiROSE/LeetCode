impl Solution {
    pub fn valid_ip_address(query_ip: String) -> String {
      let ip_str = query_ip.as_str();    
      if Self::ipv4(ip_str) {
        return "IPv4".into();
      }
      if Self::ipv6(ip_str) {
        return "IPv6".into()
      }
      "Neither".into()

    }

    fn ipv4(ip_str: &str) -> bool{
      let nums: Vec<_> = ip_str.split(".").collect();
      if nums.len() != 4 {return false;}
      for num in nums {
        if num.len() == 0 || num.len() > 3 || (num.len() > 1 && num.starts_with("0")) {return false;}
        let mut sum = 0;
        for ch in num.chars() {
            if !ch.is_ascii_digit() {
              return false;
            }
            sum = sum*10+(ch as u8 -b'0') as usize;
        }
        if sum > 255 {return false;}
      }
      true
    }

    fn ipv6(ip_str: &str) -> bool {
      let nums: Vec<_> = ip_str.split(":").collect();
      if nums.len() != 8 {return false;}
      for num in nums {
        if num.len() == 0 || num.len() > 4 {return false;}
        for ch in num.chars() {
          if !(ch >= '0' && ch <='9' || ch >= 'a' && ch <='f' || ch >= 'A' && ch <='F') {
            return false;
          }
        }
      }
      true
    }
}