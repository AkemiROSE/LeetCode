class Solution {
public:
        ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        ListNode *sum_list = NULL, *tail = NULL;
        int p = 0;
        while (l1 || l2) {
            ListNode* tmp = new ListNode(p);
            if (l1) {
                tmp->val += l1->val;
                l1 = l1->next;
            }
            if (l2) {
                tmp->val += l2->val;
                l2 = l2->next;
            }
            if (tmp->val > 9) {
                tmp->val -= 10;
                p = 1;
            }
            else {
                p = 0;
            }
            if (sum_list == NULL) {
                sum_list = tmp;
                tail = tmp;
            }
            else {
                tail->next = tmp;
                tail = tail->next;
            }
        }
        if (p > 0) {
            tail->next = new ListNode(p);
        }
        return sum_list;
    }
};