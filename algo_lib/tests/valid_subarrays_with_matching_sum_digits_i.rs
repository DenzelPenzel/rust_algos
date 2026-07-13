/*
You are given an integer array nums and an integer digit x.

A subarray nums[l..r] is considered valid if the sum of its elements
satisfies both of the following conditions:

The first digit of the sum is equal to x.
The last digit of the sum is equal to x.
Return the number of valid subarrays.

Example 1:
    Input: nums = [1,100,1], x = 1
    Output: 4
    Explanation:
        The valid subarrays are:

        nums[0..0]: sum = 1
        nums[0..1]: sum = 1 + 100 = 101
        nums[1..2]: sum = 100 + 1 = 101
        nums[2..2]: sum = 1
        Thus, the answer is 4.

Example 2:
    Input: nums = [1], x = 2
    Output: 0
    Explanation:
        The only subarray is nums[0..0] with a sum of 1, which does not satisfy the conditions.
        Thus, the answer is 0.

Constraints:
    1 <= nums.length <= 1500
    1 <= nums[i] <= 109
    1 <= x <= 9
*/

impl Solution {
    pub fn count_valid_subarrays(nums: Vec<i32>, x: i32) -> i32 {
        fn first(mut num: i64) -> i64 {
            while num >= 10 {
                num /= 10;
            }
            num
        }

        let n = nums.len() as usize;
        let x = x as i64;
        let mut pref = vec![0i64; n + 1];
        for i in 0..n {
            pref[i + 1] = pref[i] + nums[i] as i64;
        }

        let mut res = 0;

        for i in 0..n {
            for j in i..n {
                let sum = pref[j + 1] - pref[i];

                if sum % 10 == x && first(sum) == x {
                    res += 1;
                }
            }
        }

        res
    }
}
