/*
You are given an integer array nums.

Return the length of the longest strictly increasing subsequence in nums whose bitwise AND is non-zero. If no such subsequence exists, return 0.

Example 1:
    Input: nums = [5,4,7]
    Output: 2
    Explanation:
        One longest strictly increasing subsequence is [5, 7]. The bitwise AND is 5 AND 7 = 5, which is non-zero.

Example 2:
    Input: nums = [2,3,6]
    Output: 3
    Explanation:
        The longest strictly increasing subsequence is [2, 3, 6]. The bitwise AND is 2 AND 3 AND 6 = 2, which is non-zero.

Example 3:
    Input: nums = [0,1]
    Output: 1
    Explanation:
        One longest strictly increasing subsequence is [1]. The bitwise AND is 1, which is non-zero.

    
Constraints:
    1 <= nums.length <= 105
    0 <= nums[i] <= 109​​​​​​​
*/


struct Solution;

impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        let mut res = 0;

        for i in 0..30 {
            let A: Vec<i32> = nums
                .iter()
                .cloned()
                .filter(|&x| (x & (1 << i)) != 0)
                .collect();

            if A.is_empty() {
                continue;
            }
            
            let mut lst: Vec<i32> = Vec::new();
            for x in A {
                let idx = lst.partition_point(|&v| v < x);
                if idx < lst.len() {
                    lst[idx] = x;
                } else {
                    lst.push(x);
                }
            }

            if lst.len() as i32 > res {
                res = lst.len() as i32;
            }
        }        

        res
    }
}