void helper(vector<vector<char>>& grid, int i, int j, int col, int row, vector<bool>& seen) {
    if (grid[i][j] == '0' || seen[i * col + j] == true) return;
    seen[i * col + j] = true;
    if (i > 0)helper(grid, i - 1, j, col, row, seen);
    if (j > 0)helper(grid, i, j - 1, col, row, seen);
    if(i < row-1)helper(grid, i + 1, j, col, row, seen);
    if (j < col-1)helper(grid, i, j + 1, col, row, seen);
}
class Solution {
public:
int numIslands(vector<vector<char>>& grid) {
    int row = grid.size();
    int col = grid[0].size();
    int count = 0;
    vector<bool> seen(row * col, false);
    for (int i = 0; i < row; i++) {
        for (int j = 0; j < col; j++) {
            if (grid[i][j] == '1' && seen[i * col + j] == false) {
                count++;
                helper(grid, i, j, col, row, seen);
            }
        }
    }
    return count;
}
};