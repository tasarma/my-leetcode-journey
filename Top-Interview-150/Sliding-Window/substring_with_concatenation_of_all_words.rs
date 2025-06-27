// Problem: 30. Sum Substring with Concatenation of All Words
// Technique: Sliding Window
// Link: https: https://leetcode.com/problems/substring-with-concatenation-of-all-words/description/?envType=study-plan-v2&envId=top-interview-150

use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut result = Vec::new();
        let word_len = words[0].len();
        let word_count = words.len();
        let chunk_len = word_len * word_count;
        let s_len = s.len();

        let mut word_freq: HashMap<&str, usize> = HashMap::new();
        for word in &words {
            *word_freq.entry(word.as_str()).or_insert(0) += 1;
        }

        let s_bytes = s.as_bytes(); // for efficient slicing

        for i in 0..word_len {
            let mut left = i;
            let mut right = i;
            let mut count = 0;
            let mut current_count: HashMap<&str, usize> = HashMap::new();

            while right + word_len <= s_len {
                let word = &s[right..right + word_len];
                right += word_len;

                if word_freq.contains_key(word) {
                    *current_count.entry(word).or_insert(0) += 1;
                    count += 1;

                    while current_count[word] > word_freq[word] {
                        let left_word = &s[left..left + word_len];
                        if let Some(entry) = current_count.get_mut(left_word) {
                            *entry -= 1;
                        }
                        count -= 1;
                        left += word_len;
                    }

                    if count == word_count {
                        result.push(left as i32);
                    }
                } else {
                    current_count.clear();
                    count = 0;
                    left = right;
                }
            }
        }

        result
    }
}
