impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
      let row = matrix.len();
      let col = matrix[0].len();
      helper(&matrix, target,0, col-1)
    }
  }
  
  fn helper(matrix: &Vec<Vec<i32>>, target: i32, i: usize, j: usize) ->bool{
    if matrix[i][j] == target {return true;}
    else if j > 0 && matrix[i][j] > target {return helper(matrix, target,i, j-1);}
    else if i < matrix.len()-1 &&  matrix[i][j] < target {return helper(matrix, target,i+1,j);}
    else{false}
  }