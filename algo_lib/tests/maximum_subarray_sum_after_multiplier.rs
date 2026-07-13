/*
You are given an integer array nums and a positive integer k.

You must choose exactly one subarray of nums and perform exactly one of the following operations:

Multiply each number in the chosen subarray by k.
Divide each number in the chosen subarray by k.
When dividing a positive number by k, use the floor value of the division result.
When dividing a negative number by k, use the ceiling value of the division result.
Return the maximum possible sum of a non-empty subarray in the resulting array.

Note that the subarray chosen for the
operation and the subarray chosen for the sum may be different.

Example 1:
    Input: nums = [1,-2,3,4,-5], k = 2
    Output: 14
    Explanation:
        Multiply each number in the subarray [3, 4] by 2.
        This results in nums = [1, -2, 6, 8, -5].
        The subarray with the largest sum is [6, 8], so the output is 6 + 8 = 14.

Example 2:
    Input: nums = [-5,-4,-3], k = 2
    Output: -1
    Explanation:
        Divide each number in the subarray [-3] by 2.
        This results in nums = [-5, -4, -1].
        The subarray with the largest sum is [-1], so the output is -1.

Constraints:
    1 <= nums.length <= 105
    -105 <= nums[i] <= 105
    1 <= k <= 105
*/

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        fn solve(nums: &[i32], change: impl Fn(i64) -> i64) -> i64 {
            let first = nums[0] as i64;
            let neg = i64::MIN / 4;

            let mut dp0 = first;
            let mut dp1 = change(first);
            let mut dp2 = neg;

            let mut res = dp1;
            let n = nums.len();

            for i in 1..n {
                let num = nums[i] as i64;
                let new_num = change(num);

                // default Kadane
                let nx0 = (dp0 + num).max(num);

                // default Kadana when we take change number
                let nx1 = (dp0 + new_num).max(dp1 + new_num).max(new_num);
                let nx2 = (dp2 + num).max(dp1 + num);

                dp0 = nx0;
                dp1 = nx1;
                dp2 = nx2;

                res = res.max(nx1).max(nx2);
            }

            res
        }

        let k = k as i64;
        let multiply = solve(&nums, |x| x * k);
        let divide = solve(&nums, |x| x / k);

        multiply.max(divide)
    }
}
