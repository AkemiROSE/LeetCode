impl Solution {
    pub fn remove_kdigits(num: String, mut k: i32) -> String {
        if num.len() as i32 == k {return "0".into()}
        let mut stack = vec![];
        for ch in num.chars() {
          while k > 0 && stack.last().is_some() {
            if *stack.last().unwrap() > ch  {stack.pop();k-=1;}
            else{break;}
          }
          if stack.len() > 0 || !ch.eq(&'0'){stack.push(ch);}
        }
        while k > 0 {stack.pop(); k-=1;}
        if stack.len() == 0 {stack.push('0');}
        stack.into_iter().collect()
    }
  }