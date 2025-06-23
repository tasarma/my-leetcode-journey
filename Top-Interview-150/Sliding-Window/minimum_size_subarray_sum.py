# Problem: 209. Minimum Size Subarray Sum
# Technique: Sliding Window
# Link: https://leetcode.com/problems/minimum-size-subarray-sum/description/?envType=study-plan-v2&envId=top-interview-150


class Solution:
    def minSubArrayLen(self, target: int, nums: List[int]) -> int:
        left, windows_sum, min_length = 0, 0, float('inf')

        for right in range(len(nums)):
            windows_sum += nums[right]

            while windows_sum >= target:
                min_length = min(min_length, right - left + 1)
                windows_sum -= nums[left]
                left += 1
            
        return 0 if min_length == float('inf') else min_length
