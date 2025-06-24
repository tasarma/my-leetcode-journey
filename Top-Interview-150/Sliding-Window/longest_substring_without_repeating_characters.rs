// Problem: 3. Longest Substring Without Repeating Characters
// Technique: Sliding Window
// Link: https://leetcode.com/problems/longest-substring-without-repeating-characters/description/?envType=study-plan-v2&envId=top-interview-150
//

use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut max_len = 0;
        let mut windows_substring = HashSet::new();

        for (right, &ch) in chars.iter().enumerate() {
            while windows_substring.contains(&ch) {
                windows_substring.remove(&chars[left]);
                left += 1;
            }

            windows_substring.insert(ch);
            max_len = max_len.max(right - left + 1);
        }

        max_len as i32
    }
}
