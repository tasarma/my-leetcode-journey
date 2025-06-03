// Problem: Is Subsequence
// Technique: Two Pointers
// Link: https://leetcode.com/problems/is-subsequence/?envType=study-plan-v2&envId=top-interview-150

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s_chars = s.chars();
        let mut current = s_chars.next();

        for c in t.chars() {
            if let Some(ch) = current {
                if c == ch {
                    current = s_chars.next();
                }
            } else {
                break;
            }
        }

        current.is_none()
    }
}
