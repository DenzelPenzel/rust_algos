/*
You are given an integer array nums and an integer digit.

Return the total number of times digit appears 
in the decimal representation of all elements in nums.

Example 1:
    Input: nums = [12,54,32,22], digit = 2
    Output: 4
    Explanation:
        The digit 2 appears once in 12 and 32, and twice in 22. Thus, the total number of times digit 2 appears is 4.

Example 2:
    Input: nums = [1,34,7], digit = 9
    Output: 0
    Explanation:
        The digit 9 does not appear in the decimal representation of any element in nums, so the total number of times digit 9 appears is 0.

Constraints:
    1 <= nums.length <= 1000
    1 <= nums[i] <= 106​​​​​​​
    0 <= digit <= 9
*/


impl Solution {
    pub fn count_digit_occurrences(nums: Vec<i32>, digit: i32) -> i32 {
        let mut res = 0;
        let target = digit as u32;
        for &num in &nums {
            let mut n = num.unsigned_abs();
            if n == 0 {
                if target == 0 {
                    res += 1
                }
                continue;
            }
            while n > 0 {
                if n % 10 == target {
                    res += 1;
                }
                n /= 10; 
            }
        }
        res
    }
}
