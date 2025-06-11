# Problem: Two Sum II
# Technique: Two Pointers
# Link: https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/?envType=study-plan-v2&envId=top-interview-150

class Solution:
    def twoSum(self, numbers: List[int], target: int) -> List[int]:
        start = 0
        end = len(numbers) - 1

        while start < end:
            curr_sum = numbers[start] + numbers[end]
            if curr_sum == target:
                return [start + 1,  end + 1]
            elif curr_sum < target:
                start += 1
            else:
                end -= 1

