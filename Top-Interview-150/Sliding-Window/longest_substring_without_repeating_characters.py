# Problem: 3. Longest Substring Without Repeating Characters
# Technique: Sliding Window
# Link: https://leetcode.com/problems/longest-substring-without-repeating-characters/description/?envType=study-plan-v2&envId=top-interview-150


class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        left =  0
        windows_substring = set()
        max_len = 0

        for right in range(len(s)):
            while s[right] in windows_substring:
                windows_substring.remove(s[left])
                left  += 1

            windows_substring.add(s[right])
            max_len = max(max_len, right - left + 1)
        
        return max_len
