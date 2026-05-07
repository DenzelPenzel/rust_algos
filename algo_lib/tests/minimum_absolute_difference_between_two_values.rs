/*
You are given an integer array nums consisting only of 0, 1, and 2.

A pair of indices (i, j) is called valid if nums[i] == 1 and nums[j] == 2.

Return the minimum absolute difference between i and j among all valid pairs. If no valid pair exists, return -1.

The absolute difference between indices i and j is defined as abs(i - j).

Example 1:
    Input: nums = [1,0,0,2,0,1]
    Output: 2
    Explanation:
        The valid pairs are:
        (0, 3) which has absolute difference of abs(0 - 3) = 3.
        (5, 3) which has absolute difference of abs(5 - 3) = 2.
        Thus, the answer is 2.

Example 2:
    Input: nums = [1,0,1,0]
    Output: -1
    Explanation:
        There are no valid pairs in the array, thus the answer is -1.

Constraints:
    1 <= nums.length <= 100
    0 <= nums[i] <= 2
*/


impl Solution {
    pub fn min_absolute_difference(nums: Vec<i32>) -> i32 {
        let mut last_1: i32 = -1;
        let mut last_2: i32 = -1;
        let mut min_diff = i32::MAX;

        for (i, &num) in nums.iter().enumerate() {
            let idx = i as i32;
            if num == 1 {
                last_1 = idx;
                if last_2 != -1 {
                    min_diff = min_diff.min(idx - last_2);
                }
            } else if num == 2 {    
                last_2 = idx;
                if last_1 != -1 {
                    min_diff = min_diff.min(idx - last_1);
                }
            }
        }

        if min_diff == i32::MAX {
            -1
        } else {
            min_diff
        }
    }
}

