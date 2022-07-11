void fun(int index, vector<int> nums, vector<vector<int>> &res) {
    if(index == nums.size() - 1) res.push_back(nums);
    for(int i = index; i < nums.size(); i++) {
        swap(nums[index], nums[i]);
        fun(index+1, nums, res);
    }
}

class Solution {
public:
    vector<vector<int>> permute(vector<int>& nums) {
        vector<vector<int>> res{};
        fun(0, nums, res);
        return res;
    }
};