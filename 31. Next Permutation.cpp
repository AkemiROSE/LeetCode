class Solution {
public:
	void nextPermutation(vector<int>& nums) {
		int right = nums.size() - 1;
		int left = right - 1;
		while (left >= 0 && nums[left] >= nums[left + 1])left--;
		if (left >= 0) {
			while (nums[right] <= nums[left])right--;
			swap(nums[left], nums[right]);
		}
		reverse(nums.begin() + left + 1, nums.end());
	}
};

/*
example: 1 2 8 4 3
step 1: find a ascending pair from right to left, we get (2, 8).
step 2: numbers after 2 must be orderly, find the smallest number that bigger than 2, we get 3, then swap 2 and 3.
step 3: reverse the sequence after nmber 2, we get '1 3 2 4 8'. 
*/