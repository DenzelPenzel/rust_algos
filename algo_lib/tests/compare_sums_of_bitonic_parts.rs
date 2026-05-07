/*
You are given a bitonic array nums of length n.

Split the array into two parts:

Ascending part: from index 0 to the peak element (inclusive).
Descending part: from the peak element to index n - 1 (inclusive).
The peak element belongs to both parts.

Return:

0 if the sum of the ascending part is greater.
1 if the sum of the descending part is greater.
-1 if both sums are equal.

Notes:

A bitonic array is an array that is strictly increasing up to a 
single peak element and then strictly decreasing.

An array is said to be strictly increasing if each element 
is strictly greater than its previous one (if exists).

An array is said to be strictly decreasing if each element 
is strictly smaller than its previous one (if exists).
 
Example 1:
    Input: nums = [1,3,2,1]
    Output: 1
    Explanation:
        Peak element is nums[1] = 3
        Ascending part = [1, 3], sum is 1 + 3 = 4
        Descending part = [3, 2, 1], sum is 3 + 2 + 1 = 6
        Since the descending part has a larger sum, return 1.

Example 2:
    Input: nums = [2,4,5,2]
    Output: 0
    Explanation:
        Peak element is nums[2] = 5
        Ascending part = [2, 4, 5], sum is 2 + 4 + 5 = 11
        Descending part = [5, 2], sum is 5 + 2 = 7
        Since the ascending part has a larger sum, return 0.

Example 3:
    Input: nums = [1,2,4,3]
    Output: -1
    Explanation:
        Peak element is nums[2] = 4
        Ascending part = [1, 2, 4], sum is 1 + 2 + 4 = 7
        Descending part = [4, 3], sum is 4 + 3 = 7
        Since both parts have equal sums, return -1.
    
Constraints:
    3 <= n == nums.length <= 105
    1 <= nums[i] <= 109
    nums is a bitonic array.
*/


use std::cmp::Ordering;

impl Solution {
    pub fn compare_bitonic_sums(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut peak_id = 0;

        while peak_id + 1 < n && nums[peak_id] < nums[peak_id + 1] {
            peak_id += 1
        }

        let left_s: i64 = nums[..=peak_id]
            .iter()
            .map(|&x| x as i64)
            .sum();

        let right_s: i64 = nums[peak_id..]
            .iter()
            .map(|&x| x as i64)
            .sum();

        match left_s.cmp(&right_s) {
            Ordering::Greater => 0,
            Ordering::Less => 1,
            Ordering::Equal => -1,
        }
    }
}
