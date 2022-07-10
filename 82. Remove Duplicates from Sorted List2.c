struct ListNode* deleteDuplicates(struct ListNode* head) {
    if (!head || !head->next) return head;
    struct ListNode* pre = NULL;
    struct ListNode* cur = head;
    struct ListNode* next = head->next;

    while (next) {
        if (cur->val != next->val) {
            pre = cur;
            cur = cur->next;
            next = next->next;
        }
        else {
            while (cur->val == next->val) {
                next = next->next;
                if (!next)break;
            }
            if (next) {
                cur = next;
                next = next->next;
                if (pre) { pre->next = cur; }
                if (!pre) { head = cur; }
            }
            else {
                if (pre)pre->next = NULL;
                else return NULL;
            }
        }
    }
    return head;
}