bool match(vector<int> & vec, const int& targetsum) {
    int long sum{};
    for (int n : vec) { sum += n; }
    return sum == targetsum ? true : false;
}

void helper113(TreeNode* root, int& targetSum, vector<vector<int>> &res, vector<int> &tmp) {
    if (!root)return;
    tmp.push_back(root->val);
    if (!root->left && !root->right) {
        if (match(tmp, targetSum)) { res.push_back(tmp); }
    }
    else{
        helper113(root->left, targetSum, res, tmp);
        helper113(root->right, targetSum, res, tmp);
    }
    tmp.pop_back();
}
class Solution {
public:
    vector<vector<int>> pathSum(TreeNode* root, int targetSum) {
        vector<vector<int>> res{};
        vector<int> tmp{};
        helper113(root, targetSum, res, tmp);
        return res;
    }
};