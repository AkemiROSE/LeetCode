bool hasCycle(struct ListNode *head) {
    struct ListNode *p1 = head;
    struct ListNode *p2 = head;
    while(p2!= NULL && p2->next != NULL ){
        p1 = p1->next;
        p2 = p2->next->next;
        if(p1 == p2){return true;}
    }
    return false;
}