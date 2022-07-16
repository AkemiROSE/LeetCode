struct ListNode* swapPairs(struct ListNode* head) {
	if (!head || !head->next) return head;
	struct ListNode* first = head;
	struct ListNode* second = head->next;
	struct ListNode* tmp = swapPairs(second->next);
	second->next = first;
	first->next = tmp;
	return second;
}