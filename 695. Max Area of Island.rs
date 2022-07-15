impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
      let row = grid.len();
      let col = grid[0].len();
      let mut seen = vec![vec![false; col]; row];
      let mut res = 0;
      for i in 0..row {
        for j in 0..col {
            if grid[i][j] == 1 && seen[i][j] == false {
              res = std::cmp::max(res, Self::helper(&grid, i, j, row, col, &mut seen));
            }
        }
      }
      res as _
    }
  
    fn helper(grid: &Vec<Vec<i32>>, i: usize, j: usize, row: usize, col: usize, seen: &mut Vec<Vec<bool>>) -> usize{
      if i >= row || j >= col || grid[i][j] == 0||seen[i][j] == true {return 0;}
      seen[i][j] = true;
      Self::helper(grid, i-1, j, row, col, seen) + 
      Self::helper(grid, i+1, j, row, col, seen) +
      Self::helper(grid, i, j+1, row, col, seen) +
      Self::helper(grid, i, j-1, row, col, seen) + 1
    }
  }