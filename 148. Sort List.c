
struct ListNode* sortList(struct ListNode* head) {
	if (!head || !head->next)return head;
	struct ListNode* fast = head->next;
	struct ListNode* slow = head;
	while (fast && fast->next) {
		fast = fast->next->next;
		slow = slow->next;
	}
	struct ListNode* second = slow->next;
	slow->next = NULL;
	struct ListNode* left = sortList(head);
	struct ListNode* right = sortList(second);

	struct ListNode* h= NULL;
	struct ListNode* cur = NULL;
	while (left && right) {
		if (left->val > right->val) {
			if (h == NULL) { h = right; cur = h; }
			else {
				cur->next = right;
				cur = cur->next;
			}
			right = right->next;
		}
		else {
			if (h == NULL) { h = left; cur = h; }
			else {
				cur->next = left;
				cur = cur->next;
			}
			left = left->next;
		}
	}
	if (left)cur->next = left;
	if (right)cur->next = right;

	return h;
	
}