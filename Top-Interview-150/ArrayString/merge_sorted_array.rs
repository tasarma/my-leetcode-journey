// Problem: 88. Merge Sorted Array
// Technique:  Two Pointer
// Link: https://leetcode.com/problems/merge-sorted-array/description/?envType=study-plan-v2&envId=top-interview-150

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut idx1 = m as isize - 1;
        let mut idx2 = n as isize - 1;
        let mut idx = (m + n) as isize - 1;

        while idx2 >= 0 {
            if idx1 >= 0 && nums1[idx1 as usize] > nums2[idx2 as usize] {
                nums1[idx as usize] = nums1[idx1 as usize];
                idx1 -= 1;
            } else {
                nums1[idx as usize] = nums2[idx2 as usize];
                idx2 -= 1;
            }
            idx -= 1;
        }
    }
}
