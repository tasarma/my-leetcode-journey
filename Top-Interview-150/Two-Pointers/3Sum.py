# Problem: 3Sum
# Technique: Two Pointers
# Link: https://leetcode.com/problems/3sum/?envType=study-plan-v2&envId=top-interview-150


class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        nums.sort()
        triplets = []
        n = len(nums)

        for idx in range(len(nums)):
            # Skip duplicates
            if idx > 0 and nums[idx] == nums[idx - 1]:
                continue
            
            if nums[idx] > 0:
                break

            left = idx + 1
            right = n - 1
            while left < right:
                current_sum = nums[idx] + nums[left] + nums[right]
                if current_sum == 0:
                    triplets.append([nums[idx], nums[left], nums[right]])
                    # Skip duplicates
                    while left < right and nums[left] == nums[left + 1]:
                            left += 1
                    # Skip duplicates
                    while left < right and nums[right] == nums[right - 1]:
                            right -= 1
                    left += 1
                    right -= 1
                elif current_sum > 0:
                    right -= 1
                elif current_sum < 0:
                    left += 1

        return triplets

