impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
      let len =  bits.len();
      let mut i = 0;
      loop {
          if i >= len{break;}
          if bits[i] == 0 {
            if i == len-1 {return true;}
            i+=1;
          }
          else {i+=2;}      
        }
        false
    }
  }