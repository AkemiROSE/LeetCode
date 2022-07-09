class Solution {
public:
    string longestPalindrome(string s) {
        int i = 0;
        string res = s.substr(0, 1);
        int len = s.length();
        while (i < len) {
            int left = i;
            int right = i + 1;
            while (left >= 0 && right < len && s[left] == s[right]) { left--; right++; }
            int sublen = right - left - 1;
            if (sublen > res.length()) { res = s.substr(left + 1, sublen); }
            i++;

        }
        i = 1;
        while (i < len) {
            int left = i - 1;
            int right = i + 1;
            while (left >= 0 && right < len && s[left] == s[right]) { left--; right++; }
            int sublen = right - left - 1;
            if (sublen > res.length()) { res = s.substr(left + 1, sublen); }
            i++;
        }
        return res;
    }
};