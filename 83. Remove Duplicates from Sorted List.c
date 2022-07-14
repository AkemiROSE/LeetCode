struct ListNode* deleteDuplicates(struct ListNode* head) {
	if (!head || !head->next) { return head; }
	struct ListNode* cur = head;
	struct ListNode* next = head->next;
	while (true) {
		while (next && cur->val == next->val) { next = next->next; }
		cur->next = next;
		if (!next) break;
		cur = cur->next;
		next = cur->next;
	}
	return head;
}