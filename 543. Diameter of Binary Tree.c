int helper(struct TreeNode* root, int* max) {
    if(!root) return 0;
    
    int left_height = helper(root->left, max);
    int right_height = helper(root->right, max);
    int height = left_height > right_height ? left_height+1 : right_height+1;
    int cur_max = left_height+ right_height;
    if (cur_max > *max) *max = cur_max;
    
    return height;
}

int diameterOfBinaryTree(struct TreeNode* root){
    int max = 0;
    helper(root, &max);
    return max;
}
