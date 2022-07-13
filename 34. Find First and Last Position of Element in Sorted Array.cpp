class Solution {
public:
	vector<int> searchRange(vector<int>& nums, int target) {
		vector<int> res(2, -1);
		int left = 0, right = nums.size() - 1;
		while (left <= right) {
			int mid = left + (right - left) / 2;
			if (target == nums[mid] && (mid == 0 || target > nums[mid - 1])) {res[0] = mid;break;}
			else if (target <= nums[mid]) { right = mid - 1; }
			else { left = mid + 1; }
		}
		left = 0; right = nums.size() - 1;
		while (left <= right) {
			int mid = left + (right - left) / 2;
			if (target == nums[mid] && (mid == nums.size()-1 || target < nums[mid + 1])) {res[1] = mid;break;}
			else if (target < nums[mid]) { right = mid - 1; }
			else { left = mid + 1; }
		}
		return res;
	}
};