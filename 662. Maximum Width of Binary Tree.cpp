void helper662(struct TreeNode* root, vector<unsigned long long>& max, vector<unsigned long long>& min, int level,unsigned long long pos, bool left) {
    while(max.size() < level+1){
        max.push_back(0);
        min.push_back(18446744073709551615);
    }
    unsigned long long cur_pos = pos*2+ (left == true ? 0:1);
    if (cur_pos > max[level]) max[level] = cur_pos;
    if (cur_pos < min[level]) min[level] = cur_pos;
    if(root->left)helper662(root->left, max, min, level+1, cur_pos,true);
    if (root->right)helper662(root->right, max, min, level+1,  cur_pos,false);
    
}

class Solution {
public:
 int widthOfBinaryTree(struct TreeNode* root) {
        vector<unsigned long long> min{};
        vector<unsigned long long> max{};
        helper662(root, max, min,0, 0, true);
        int res = 0;
        for(int i =0; i < min.size(); i++) {
            int tmp = max[i] - min[i]+1;
            if (tmp > res) {res = tmp;}
        }
        return res;
    }
};