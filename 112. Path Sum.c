bool hasPathSum(struct TreeNode* root, int targetSum) {
	if (!root) {return false; }
	targetSum -= root->val;;
	if (targetSum == 0 && !root->left && !root->right) {return true; }
	bool res = false;
	res = hasPathSum(root->left, targetSum); 
	if (!res) { res = hasPathSum(root->right, targetSum); }
	return res;
}