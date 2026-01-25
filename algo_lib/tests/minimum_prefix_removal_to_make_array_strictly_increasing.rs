/*
You are given an integer array nums.

You need to remove exactly one prefix (possibly empty) from nums.

Return an integer denoting the minimum length of the 
removed prefix such that the remaining array is strictly increasing.

Example 1:
    Input: nums = [1,-1,2,3,3,4,5]
    Output: 4
    Explanation:
        Removing the prefix = [1, -1, 2, 3] leaves the remaining array [3, 4, 5] which is strictly increasing.

Example 2:
    Input: nums = [4,3,-2,-5]
    Output: 3
    Explanation:
        Removing the prefix = [4, 3, -2] leaves the remaining array [-5] which is strictly increasing.

Example 3:
    Input: nums = [1,2,3,4]
    Output: 0
    Explanation:
        The array nums = [1, 2, 3, 4] is already strictly increasing 
        so removing an empty prefix is sufficient.

Constraints:
    1 <= nums.length <= 10^5
    -10^9 <= nums[i] <= 10^9​​​​​​​
*/

use std::cmp;

struct Solution;

impl Solution {
    pub fn minimum_prefix_length(nums: Vec<i32>) -> i32 {
        nums.windows(2)
            .rposition(|w| w[0] >= w[1])
            .map(|idx| (idx + 1) as i32)
            .unwrap_or(0)
    }

    pub fn minimum_prefix_length_II(nums: Vec<i32>) -> i32 {
        let mut i = nums.len() - 1;
        while i > 0 {
            if i - 1 >= 0 && nums[i - 1] >= nums[i] {
                break;
            }
            i -= 1;
        }
        cmp::max(0, i as i32)
    }
}