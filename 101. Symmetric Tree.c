bool isSameTree(struct TreeNode* p, struct TreeNode* q) {
	if (!p && !q) { return true; }
	else if (p && q && p->val == q->val) {
		return isSameTree(p->left, q->right) && isSameTree(p->right, q->left);
	}
	else { return false; }
}

bool isSymmetric(struct TreeNode* root) {
	return isSameTree(root->left, root->right);
}