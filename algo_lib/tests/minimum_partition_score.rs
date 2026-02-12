/*
You are given an integer array nums and an integer k.

Your task is to partition nums into exactly k subarrays and return an integer denoting the minimum possible score among all valid partitions.

The score of a partition is the sum of the values of all its subarrays.

The value of a subarray is defined as sumArr * (sumArr + 1) / 2, where sumArr is the sum of its elements.

Example 1:
    Input: nums = [5,1,2,1], k = 2
    Output: 25
    Explanation:
        We must partition the array into k = 2 subarrays. One optimal partition is [5] and [1, 2, 1].
        The first subarray has sumArr = 5 and value = 5 × 6 / 2 = 15.
        The second subarray has sumArr = 1 + 2 + 1 = 4 and value = 4 × 5 / 2 = 10.
        The score of this partition is 15 + 10 = 25, which is the minimum possible score.

Example 2:
    Input: nums = [1,2,3,4], k = 1
    Output: 55
    Explanation:
        Since we must partition the array into k = 1 subarray, all elements belong to the same subarray: [1, 2, 3, 4].
        This subarray has sumArr = 1 + 2 + 3 + 4 = 10 and value = 10 × 11 / 2 = 55.​​​​​​​
        The score of this partition is 55, which is the minimum possible score.

Example 3:
    Input: nums = [1,1,1], k = 3
    Output: 3
    Explanation:
        We must partition the array into k = 3 subarrays. The only valid partition is [1], [1], [1].
        Each subarray has sumArr = 1 and value = 1 × 2 / 2 = 1.
        The score of this partition is 1 + 1 + 1 = 3, which is the minimum possible score.
    
Constraints:
    1 <= nums.length <= 1000
    1 <= nums[i] <= 104
    1 <= k <= nums.length 
*/


struct Solution;

impl Solution {
    pub fn min_partition_score(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;
        let mut prefix = vec![0i64; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + nums[i] as i64;
        }

        let mut memo = vec![vec![-1i64; n]; k + 1];

        Self::solve(0, k, n, &prefix, &mut memo)
    }

    fn solve(i: usize, k: usize, n: usize, prefix: &Vec<i64>, memo: &mut Vec<Vec<i64>>) -> i64 {
        if memo[k][i] != -1 {
            return memo[k][i];
        }

        if k == 1 {
            let s = prefix[n] - prefix[i];
            let cost = s * (s + 1) / 2;
            memo[k][i] = cost;
            return cost;
        }

        let mut min_score = i64::MAX;

        for ii in i..=(n - k) {
            let s = prefix[ii + 1] - prefix[i];
            let cost = s * (s + 1) / 2;
            let res = cost + Self::solve(ii + 1, k - 1, n, prefix, memo);

            if res != i64::MAX {
                min_score = cmp::min(min_score, res);
            }
        }

        memo[k][i] = min_score;
        min_score
    }
}

use std::cmp;

struct Solution_II;

impl Solution_II {
    pub fn min_partition_score(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;
        let mut prefix = vec![0i64; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + nums[i] as i64;
        }

        let get_cost = |i: usize, j: usize| -> i64 {
            if i > j {
                return 0;
            }
            let s = prefix[j + 1] - prefix[i];
            s * (s + 1) / 2
        };

        let mut dp_prev = vec![i64::MAX; n + 1];
        let mut dp_curr = vec![i64::MAX; n + 1];

        for i in 1..=n {
            dp_prev[i] = get_cost(0, i - 1);
        }

        dp_prev[0] = 0;

        for _split in 2..=k {
            Self::solve(&mut dp_curr, &dp_prev, 1, n, 0, n - 1, &get_cost);
            dp_prev.copy_from_slice(&dp_curr);
        }

        dp_prev[n]
    }

    fn solve<F>(
        dp_curr: &mut Vec<i64>,
        dp_prev: &Vec<i64>,
        lo: usize,
        hi: usize,
        good_lo: usize,
        good_hi: usize,
        get_cost: &F,
    ) where
        F: Fn(usize, usize) -> i64,
    {
        if lo > hi {
            return;
        }

        let mid = lo + ((hi - lo) >> 1);
        let mut best = i64::MAX;
        let mut best_k = good_lo;

        let end = cmp::min(mid - 1, good_hi);

        for k in good_lo..=end {
            if dp_prev[k] == i64::MAX {
                continue;
            }

            let val = dp_prev[k] + get_cost(k, mid - 1);
            if val < best {
                best = val;
                best_k = k;
            }
        }

        dp_curr[mid] = best;

        Self::solve(dp_curr, dp_prev, lo, mid - 1, good_lo, best_k, get_cost);
        Self::solve(dp_curr, dp_prev, mid + 1, hi, best_k, good_hi, get_cost);
    }
}
