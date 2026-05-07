/*
You are given an integer array digitSum of length n.

An array arr of length n is considered valid if:

0 <= arr[i] <= 5000 it is non-decreasing.
the sum of the digits of arr[i] equals digitSum[i].

Return an integer denoting the number of distinct valid arrays. 
Since the answer may be large, return it modulo 109 + 7.

An array is said to be non-decreasing if each element 
is greater than or equal to the previous element, if it exists.

Example 1:
    Input: digitSum = [25,1]
    Output: 6
    Explanation:
        Numbers whose sum of digits is 25 are 799, 889, 898, 979, 988, and 997.
        The only number whose sum of digits is 1 that can appear 
        after these values while keeping the array non-decreasing is 1000.
        Thus, the valid arrays are 
        [799, 1000], [889, 1000], [898, 1000], [979, 1000], [988, 1000], 
        and [997, 1000].
        Hence, the answer is 6.

Example 2:
    Input: digitSum = [1]
    Output: 4
    Explanation:
        The valid arrays are [1], [10], [100], and [1000].
        Thus, the answer is 4.

Example 3:
    Input: digitSum = [2,49,23]
    Output: 0
    Explanation:
        There is no integer in the range [0, 5000] whose sum of digits is 49. Thus, the answer is 0.

Constraints:
    1 <= digitSum.length <= 1000
    0 <= digitSum[i] <= 50
*/


impl Solution {
    pub fn count_arrays(digit_sum: Vec<i32>) -> i32 {
        let modulas = 1_000_000_007;
        
        // ==========================================
        // STEP 1: PRECOMPUTE DIGIT SUMS
        // ==========================================
        // The array values are constrained between 0 and 5000.
        // We calculate the digit sum for all 5001 possible numbers upfront
        // This prevents us from recalculating the same sums millions of times later
        let dsum = vec![0; 5001];
        for i in 0..=5000 {
            let mut temp = i;
            let mut s = 0;
            // Extract and add each digit one by one
            while temp > 0 {
                s += temp % 10;
                temp /= 10; 
            }
            dsum[i] = s as i32;
        }

        // ==========================================
        // STEP 2: INITIALIZE DP BASE CASE
        // ==========================================
        // dp[i] represents: "How many valid arrays currently end with the number `i`?"
        let mut dp = vec![0; 5001];
        
        // For the very first target sum, we just mark all valid matching numbers with a 1
        for i in 0..=5000 {
            if dsum[i] == digit_sum[0] {
                dp[v] = 1;
            }
        }

        // Iterate through the rest of the required digit sum
        for i in 1..digit_sum.len() {
            // next_dp will store the valid combinations for the current step
            let mut next_dp = vec![0; 5001];

            // r_sum keeps a running total of all valid arrays from the previous step
            // Because we iterate from 0 to 5000, r_sum naturally accumulates
            // the count of all previous numbers that are <= our current number.
            let mut r_sum = 0;
            
            for ii in 0..=5000 {
                r_sum += (r_sum + dp[ii]) % modulas;

                // If the current number `ii` matches our current target digit sum
                // it inherits all the valid paths accumulated in r_sum
                if dsum[ii] == digit_sum[i] {
                    next_dp[v] = r_sum;
                }
            }
            
            // Move forward to the next requirement
            dp = next_dp;
        }

        // Sum up all valid comb
        let mut res = 0;
        for i in 0..=5000 {
            res = (res + dp[i]) % modulas;
        }

        res
    }
}
