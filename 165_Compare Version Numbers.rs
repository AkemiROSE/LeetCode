impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
      let mut v1: Vec<_> = version1.split(".").map(|s| s.parse::<i32>().unwrap()).collect();
      let mut v2: Vec<_> = version2.split(".").map(|s| s.parse::<i32>().unwrap()).collect();
      let mut i = 0;
      let mut j = 0;
      while i < v1.len() && j < v2.len(){
  
        if v1[i] > v2[j]  {return 1;}
        else if v1[i] < v2[j] {return -1};
        i+=1;
        j+=1;
      }
      if  i < v1.len() {
        if let Some(_) = v1.iter().skip(i).position(|&n| n != 0){
          return 1;
        }
      }
      if  j < v2.len() {
        if let Some(_) = v2.iter().skip(j).position(|&n| n != 0){
          return -1;
        }
      }
      0
    }
  }