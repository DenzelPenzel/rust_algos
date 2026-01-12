/*
You are given an integer array nums.

A subarray of nums is called centered if the sum of its elements is equal 
to at least one element within that same subarray.

Return the number of centered subarrays of nums.

Example 1:
    Input: nums = [-1,1,0]
    Output: 5
    Explanation:
        All single-element subarrays ([-1], [1], [0]) are centered.
        The subarray [1, 0] has a sum of 1, which is present in the subarray.
        The subarray [-1, 1, 0] has a sum of 0, which is present in the subarray.
        Thus, the answer is 5.

Example 2:
    Input: nums = [2,-3]
    Output: 2
    Explanation:
        Only single-element subarrays ([2], [-3]) are centered.

Constraints:
    1 <= nums.length <= 500
    -105 <= nums[i] <= 105
*/

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn centered_subarrays(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let n = nums.len();
        
        for i in 0..n {
            let mut seen: HashSet<i32> = HashSet::new();
            let mut current = 0;
            for j in i..n {
                seen.insert(nums[j]);
                current += nums[j];
                if seen.contains(&current) {
                    res += 1;
                }
            }

        }

        res
    }
}

#[test]
fn run_test() {
    let nums = vec![-1, 1, 0];
    assert_eq!(Solution::centered_subarrays(nums), 5);
}