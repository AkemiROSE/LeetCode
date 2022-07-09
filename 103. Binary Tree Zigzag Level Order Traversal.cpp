
void utilfun2(TreeNode* root, vector<vector<int>>& res, int level, bool flag) {
    if (!root) return;
    while (res.size() < level) { res.push_back(vector<int>{}); }
    if (flag) {
        res[level - 1].push_back(root->val);
        utilfun2(root->left, res, level + 1, false);
        utilfun2(root->right, res, level + 1, false);
    }
    else {
        res[level - 1].insert(res[level - 1].begin(), root->val);
        utilfun2(root->left, res, level + 1, true);
        utilfun2(root->right, res, level + 1, true);
    }  
}

class Solution {
public:
vector<vector<int>> zigzagLevelOrder(TreeNode* root) {
    vector<vector<int>> res{};
    utilfun2(root, res, 1, true);
    return res;
}
};