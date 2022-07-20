impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let  row = mat.len();
        let col = mat[0].len();
        let mut res = Vec::with_capacity(row*col);
        for sum in 0..=row+col-2 {
          for x in 0..=sum {
            let mut i = x;
            let mut j = sum - i;
            if sum % 2 == 0 {std::mem::swap(&mut i, &mut j);}
            if i>= row || j >= col {continue;}
            res.push(mat[i][j]);
          }
        }
        res
    }
    
  }