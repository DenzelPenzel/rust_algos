/*
You are given an integer array nums of odd length n.

Return true if the middle element of nums appears exactly once in the array. Otherwise return false.

Example 1:
    Input: nums = [1,2,3]
    Output: true
    Explanation:
        The middle element of nums is 2, which appears exactly once.
        Thus, the answer is true.

Example 2:
    Input: nums = [1,2,2]
    Output: false
    Explanation:
        The middle element of nums is 2, which appears twice.
        Thus, the answer is false.

Constraints:
    1 <= n == nums.length <= 100
    n is odd.
    1 <= nums[i] <= 100

*/

impl Solution {
    pub fn is_middle_element_unique(nums: Vec<i32>) -> bool {
        let mid = nums.len() / 2;
        let x = nums[mid];
        nums.iter().filter(|&&xx| xx == x).count() == 1
    }
}
