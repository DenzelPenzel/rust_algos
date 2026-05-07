/*
Given an unsorted array of integers nums, 
return the length of the longest consecutive elements sequence.

You must write an algorithm that runs in O(n) time.

Example 1:
    Input: nums = [100,4,200,1,3,2]
    Output: 4
    Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.

Example 2:
    Input: nums = [0,3,7,2,5,8,4,6,0,1]
    Output: 9

Example 3:
    Input: nums = [1,0,1,2]
    Output: 3
    
Constraints:
    0 <= nums.length <= 105
    -109 <= nums[i] <= 109
*/

use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut mp: HashSet<i32> = nums.into_iter().collect();
        let mut res = 0;

        for &x in &mp {
            if !mp.contains(&(x - 1)) {
                let mut xx = x;
                let mut res2 = 1;

                while mp.contains(&(xx + 1)) {
                    xx += 1;
                    res2 += 1;
                }
                res = res.max(res2);
            }
        }
    
        res
    }
}