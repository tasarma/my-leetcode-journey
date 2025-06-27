# Problem: 30. Sum Substring with Concatenation of All Words
# Technique: Sliding Window
# Link: https: https://leetcode.com/problems/substring-with-concatenation-of-all-words/description/?envType=study-plan-v2&envId=top-interview-150


from collections import Counter, defaultdict

class Solution:
    def findSubstring(self, s: str, words: List[str]) -> List[int]:
        word_len = len(words[0])
        word_count = len(words)
        chunk_len = word_len * word_count
        word_freq = Counter(words)
        result = []

        for i in range(word_len):
            left = i
            right = i
            current_count = defaultdict(int)
            count = 0

            while right + word_len <= len(s):
                word = s[right:right + word_len]
                right += word_len

                if word in word_freq:
                    current_count[word] += 1
                    count += 1

                    while current_count[word] > word_freq[word]:
                        left_word = s[left:left + word_len]
                        current_count[left_word] -= 1
                        count -= 1
                        left += word_len

                    if count == word_count:
                        result.append(left)
                else:
                    current_count.clear()
                    count = 0
                    left = right

        return result

