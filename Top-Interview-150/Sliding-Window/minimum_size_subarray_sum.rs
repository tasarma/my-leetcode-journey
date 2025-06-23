// Problem: 209. Minimum Size Subarray Sum
// Technique: Sliding Window
// Link: https://leetcode.com/problems/minimum-size-subarray-sum/description/?envType=study-plan-v2&envId=top-interview-150
//

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut windows_sum = 0;
        let mut min_length = nums.len() + 1;

        for right in 0..nums.len() {
            windows_sum += nums[right];

            while windows_sum >= target {
                min_length = min_length.min(right - left + 1);
                windows_sum -= nums[left];
                left += 1;
            }
        }

        if min_length > nums.len() {
            0
        } else {
            min_length as i32
        }
    }
}
