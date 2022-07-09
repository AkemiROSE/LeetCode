struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let characters = s.into_bytes();
        let mut max = 0;
        let mut cur_max = 0;
        let len = characters.len();
        let mut left = 0;
        let mut index = vec![len;128];
        for i in 0..len {
            let c = characters[i];
            if index[c as usize] < len {
                let pos = index[c as usize];
                for j in left..pos {
                    index[characters[j] as usize] = len;
                }
                left = pos+1;
                max = std::cmp::max(max, cur_max);
                cur_max = i-left+1;
            }
            else{
                cur_max+=1;
            }
            index[c as usize] = i;
        }
        std::cmp::max(max, cur_max) as i32
    }
}
