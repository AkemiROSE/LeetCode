class Solution {
public:
	vector<int> spiralOrder(vector<vector<int>>& matrix) {
		vector<int> res;
		int x_left = 0, x_right = matrix[0].size(), y_up = 0, y_down = matrix.size();
		while (true) {
			for (int j = x_left; j < x_right; j++) {
				res.push_back(matrix[y_up][j]);
			}
			++y_up;
			if (y_up == y_down)break;
			for (int i = y_up; i < y_down; i++) {
				res.push_back(matrix[i][x_right - 1]);
			}
			--x_right;
			if (x_left == x_right)break;
			for (int j = x_right - 1; j >= x_left; j--) {
				res.push_back(matrix[y_down - 1][j]);
			}
			--y_down;
			if (y_up == y_down)break;
			for (int i = y_down - 1; i >= y_up; i--) {
				res.push_back(matrix[i][x_left]);
			}
			++x_left;
			if (x_left == x_right)break;
		}
		return res;
	}
};