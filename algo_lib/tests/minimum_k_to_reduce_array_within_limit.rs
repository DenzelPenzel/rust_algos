/*
You are given a positive integer array nums.

For a positive integer k, define nonPositive(nums, k) as the minimum number of operations 
needed to make every element of nums non-positive. In one operation, you can choose an index i and reduce nums[i] by k.

Return an integer denoting the minimum value of k such that nonPositive(nums, k) <= k2.

Example 1:
    Input: nums = [3,7,5]
    Output: 3
    Explanation:
        When k = 3, nonPositive(nums, k) = 6 <= k2.

        Reduce nums[0] = 3 one time. nums[0] becomes 3 - 3 = 0.
        Reduce nums[1] = 7 three times. nums[1] becomes 7 - 3 - 3 - 3 = -2.
        Reduce nums[2] = 5 two times. nums[2] becomes 5 - 3 - 3 = -1.

Example 2:
    Input: nums = [1]
    Output: 1
    Explanation:
        When k = 1, nonPositive(nums, k) = 1 <= k2.

        Reduce nums[0] = 1 one time. nums[0] becomes 1 - 1 = 0.
    
Constraints:
    1 <= nums.length <= 105
    1 <= nums[i] <= 105

*/


struct Solution;

impl Solution {
    pub fn minimum_k(nums: Vec<i32>) -> i32 {
        let mut lo = 1;
        let mut hi = 1 << 30;
        let mut res = hi;

        while lo < hi {
            let mid = lo + ((hi - lo) >> 1);
            let mut ops: i64 = 0;

            for &x in &nums {
                ops += (x as i64 + mid - 1) / mid;
            }

            if ops <= mid * mid {
                res = mid;
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        res as i32
    }
}
