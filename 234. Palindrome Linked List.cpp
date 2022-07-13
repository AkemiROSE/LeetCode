class Solution {
public:
    bool isPalindrome(ListNode* head) {
        if (!head || !head->next) return true;
        ListNode* slow = head;
        ListNode* fast = head;
        while (fast && fast->next) {
            slow = slow->next;
            fast = fast->next->next;
        }
        ListNode* mid = fast == nullptr ? slow : slow->next;
        vector<int> stack{};
        while(head !=slow){
            stack.push_back(head->val);
            head = head->next;
        }
        while (mid) {
            if (stack.back() != mid->val) { return false; }
            stack.pop_back();
            mid = mid->next;
        }
        return true;
    }
};