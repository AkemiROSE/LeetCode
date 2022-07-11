class Solution {
public:
	vector<string> generateParenthesis(int n) {
		vector<string> res;
		
		genpair(res, "", n, n);
		return res;
	}
	void genpair(vector<string>& res, string tmp,int l, int r) {
		if (l == 0 && r == 0) { res.push_back(tmp); return; }
		
		if (l > 0) { genpair(res, tmp+"(", l - 1, r); }
		if (r > l) { genpair(res, tmp + ")", l, r - 1); }
	}
};