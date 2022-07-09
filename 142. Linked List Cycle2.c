struct ListNode *detectCycle(struct ListNode *head) {
    if(head == NULL) return NULL; 
    struct ListNode *fast = head;
    struct ListNode *slow = head;
    while(fast != NULL && fast->next != NULL) {
        fast = fast->next->next;
        slow = slow->next;
        if(slow == fast) break;
    }
    if(fast == slow && fast->next != NULL){
        slow = head;
        while(slow != fast) {
            slow = slow->next;
            fast = fast->next;
        }
        return slow;
    }
    return NULL;
}