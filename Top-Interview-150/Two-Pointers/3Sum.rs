// Problem: 3Sum
// Technique: Two Pointers
// Link: https://leetcode.com/problems/3sum/?envType=study-plan-v2&envId=top-interview-150

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut triplets: Vec<Vec<i32>> = Vec::new();
        let n = nums.len();

        for idx in 0..n {
            if idx > 0 && nums[idx] == nums[idx - 1] {
                continue;
            }

            if nums[idx] > 0 {
                break;
            }

            let mut left = idx + 1;
            let mut right = n - 1;

            while left < right {
                let current_sum = nums[idx] + nums[left] + nums[right];
                if current_sum == 0 {
                    triplets.push(vec![nums[idx], nums[left], nums[right]]);

                    // Skip duplicates
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1
                    }
                    // Skip duplicates
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1
                    }
                    left += 1;
                    right -= 1;
                } else if current_sum > 0 {
                    right -= 1;
                } else if current_sum < 0 {
                    left += 1;
                }
            }
        }

        triplets
    }
}
