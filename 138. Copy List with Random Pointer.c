struct Node* newNode(int val) {
	struct Node* node = (struct Node*)malloc(sizeof(struct Node));
	node->val = val;
	node->next = NULL;
	node->random = NULL;
	return node;
}

struct Node* copyRandomList(struct Node* head) {
    if(!head)return NULL;
	struct Node* p = head;
	while (p) {
		struct Node* node = newNode(p->val);
		node->next = p->next;
		p->next = node;
		p = p->next->next;
	}
	p = head;
	while (p) {
		if (p->random) {
			p->next->random = p->random->next;
		}
		p = p->next->next;
	}
	p = head->next;
	struct Node* q = head->next;
	while (p) {     
		head->next = p->next;
		if(head->next)p->next = head->next->next;
		head = head->next;
		p = p->next;
	}
	return q;
}