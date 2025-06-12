/ Problem: Container With Most Water
// Technique: Two Pointers
// Link: https://leetcode.com/problems/container-with-most-water/description/?envType=study-plan-v2&envId=top-interview-150

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::cmp::{max, min};

        let (mut left, mut right) = (0, height.len() - 1);
        let mut max_capacity = 0;

        while left < right {
            let width = (right - left) as i32;
            let h = min(height[left], height[right]);
            max_capacity = max(max_capacity, h * width);

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_capacity
    }
}
