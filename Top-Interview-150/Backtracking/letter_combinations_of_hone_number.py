# Problem: 17. Letter Combinations of a Phone Number
# Technique: Backtracking
# Link: https://leetcode.com/problems/letter-combinations-of-a-phone-number/?envType=study-plan-v2&envId=top-interview-150

from itertools import product

class Solution:
    def letterCombinations(self, digits: str) -> List[str]:
        maps = {
            '2': 'abc',
            '3': 'def',
            '4': 'ghi',
            '5': 'jkl',
            '6': 'mno',
            '7': 'pqrs',
            '8': 'tuv',
            '9': 'wxyz'
        }
        
        if len(digits) == 0:
            return []
        elif len(digits) == 1:
            return list(maps[digits])
        else:            
            letters = [maps[d] for d in digits]
            combinations = product(*letters)
            return [''.join(c) for c in combinations]


