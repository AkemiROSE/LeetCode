class Solution {
public:
	vector<vector<int>> combinationSum(vector<int>& candidates, int target) {
		vector<vector<int>> res;
		vector<int> tmp;
        std::sort(candidates.begin(), candidates.end());
		for (int i = 0; i < candidates.size(); i++) {
			if (candidates[i] > target)break;
			backtrace(candidates, i, target, tmp, res);
		}
        return res;
	} 
	void backtrace(vector<int>& candidates, int index, int target, vector<int>& tmp, vector<vector<int>>& res) {
		int t = 0;
		while (true) {
			tmp.push_back(candidates[index]);
			t++;
			int sum = std::accumulate(tmp.begin(), tmp.end(), 0);
			if (sum == target) {
				res.push_back(tmp);
			}
			else if (sum < target) {
				for (int i = index + 1; i < candidates.size() && candidates[i] < target; i++) {
					backtrace(candidates, i, target, tmp, res);
				}
			}
            else{
                break;
            }
		}
		while (t > 0) {
			tmp.pop_back();
			t--;
		}		
	}
};