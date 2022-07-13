class Solution {
public:
	vector<vector<int>> combinationSum2(vector<int>& candidates, int target) {
		vector<vector<int>> res;
		vector<int> tmp;
		
		std::sort(candidates.begin(), candidates.end());
		for (int i = 0; i < candidates.size(); i++) {
			if (candidates[i] > target)break;
			if(i==0||candidates[i]!=candidates[i-1])backtrace(candidates, i, target, tmp, res);
		}
		return res;
	} 
	void backtrace(vector<int>& candidates, int index, int target, vector<int>& tmp, vector<vector<int>>& res) {		
		
		tmp.push_back(candidates[index]);
		int sum = std::accumulate(tmp.begin(), tmp.end(), 0);
		if (sum == target) {
			res.push_back(tmp);
		}
		else if (sum < target) {
			for (int i = index + 1; i < candidates.size(); i++) {
				if (i == index+1 || candidates[i] != candidates[i - 1])backtrace(candidates, i, target, tmp, res);
			}
		}
		tmp.pop_back();
	}
};