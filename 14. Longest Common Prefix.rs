impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
      let mut res = String::new();
      if strs.len() == 0 {return res;}
      if strs.len() == 1 {return strs.get(0).unwrap().to_owned();}
      let tmp = strs.get(0).unwrap();
      let mut index = 0;
      for s in tmp.chars(){
        let mut flag = true;
        for i in 1..strs.len() {
          if strs.get(i).unwrap().chars().nth(index) != Some(s){flag = false;break;}
        }
        if flag{res.push(s); index +=1;}
        else{break;}
      }
      res
    }
  }