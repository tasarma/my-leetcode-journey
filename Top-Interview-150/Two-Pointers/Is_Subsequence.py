# Problem: Is Subsequence
# Technique: Two Pointers
# Link: https://leetcode.com/problems/is-subsequence/description/?envType=study-plan-v2&envId=top-interview-150

class Solution:
    def isSubsequence(self, s: str, t: str) -> bool:
        pr_s = 0
        pr_t = 0

        while pr_t < len(t) and pr_s < len(s):
            if t[pr_t] == s[pr_s]:
                pr_s +=  1
            pr_t +=  1
        
        return pr_s == len(s)

