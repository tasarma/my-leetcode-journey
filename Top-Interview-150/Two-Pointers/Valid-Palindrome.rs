// Problem: Valid Palindrome
// Technique: Two Pointers
// Link: https://leetcode.com/problems/valid-palindrome/description/?envType=study-plan-v2&envId=top-interview-150

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut right = chars.len() - 1;

        while left < right {
            if !chars[left].is_alphanumeric() {
                left += 1;
            } else if !chars[right].is_alphanumeric() {
                right -= 1;
            } else if chars[left].to_lowercase().ne(chars[right].to_lowercase()) {
                return false;
            } else {
                left += 1;
                right -= 1;
            }
        }
        true
    }
}

// let alphanumeric: Vec<char> = s
//     .chars()
//     .filter(|c| c.is_alphanumeric())
//     .map(|c|  c.to_ascii_lowercase())
//     .collect();

// alphanumeric ==  alphanumeric.iter().rev().cloned().collect::<Vec<_>>()
