struct TreeNode* invertTree(struct TreeNode* root){
    if(root == NULL || !root->left && !root->right) return root;
    struct TreeNode *left = root->left, *right = root->right;
    root->right = invertTree(left);
    root->left = invertTree(right);
    return root;
}