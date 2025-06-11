// Problem: Two Sum II
//Technique: Two Pointers
// Link: https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/?envType=study-plan-v2&envId=top-interview-150

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;

        while left < right {
            match numbers[left] + numbers[right] {
                sum if sum == target => return vec![(left + 1) as i32, (right + 1) as i32],
                sum if sum < target => left += 1,
                _ => right -= 1,
            }
        }

        Vec::new()
    }
}
