struct ListNode* getIntersectionNode(struct ListNode* headA, struct ListNode* headB) {
	struct ListNode* a = headA;
	struct ListNode* b = headB;
	while (true) {
		if (!a && b) { a = headB; }
		if (!b && a) { b = headA; }
		if (!a && !b) { return NULL; }
		if (a == b) { return a; }
        a = a->next;
		b = b->next;

	}
}