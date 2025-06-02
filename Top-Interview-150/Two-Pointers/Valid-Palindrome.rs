// Problem: Valid Palindrome
// Technique: Two Pointers
// Link: https://leetcode.com/problems/valid-palindrome/description/?envType=study-plan-v2&envId=top-interview-150

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let alphanumeric: Vec<char> = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        alphanumeric == alphanumeric.iter().rev().cloned().collect::<Vec<_>>()
    }
}
