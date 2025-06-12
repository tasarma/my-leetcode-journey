# Problem: Container With Most Water
# Technique: Two Pointers
# Link: https://leetcode.com/problems/container-with-most-water/description/?envType=study-plan-v2&envId=top-interview-150

class Solution:
    def maxArea(self, height: List[int]) -> int:
        left = 0 
        right = len(height) - 1
        max_capacity = 0

        while left < right:
            h = min(height[left], height[right])
            max_capacity = max(max_capacity, h * (right - left))
            if height[left] < height[right]:
                left += 1
            else:
                right -= 1 
    
        return max_capacity

