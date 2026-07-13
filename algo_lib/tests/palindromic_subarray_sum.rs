/*
You are given an integer array nums.

Return the maximum possible sum of a subarray of nums that is a palindrome.

Example 1:
    Input: nums = [10,10]
    Output: 20
    Explanation:
        The whole array [10,10] is a palindrome.
        Therefore, the maximum sum is 10 + 10 = 20.

Example 2:
    Input: nums = [1,2,3,2,1,5,6]
    Output: 9
    Explanation:
        The contiguous subarray [1,2,3,2,1] is a palindrome.
        Its sum is 1 + 2 + 3 + 2 + 1 = 9 and it is the maximum sum.

Example 3:
    Input: nums = [7,1,2,1,7,3,4,3,4]
    Output: 18
    Explanation:
        The contiguous subarray [7,1,2,1,7] is a palindrome.
        Its sum is 7 + 1 + 2 + 1 + 7 = 18 and it is the maximum sum.

Example 4:
    Input: nums = [1,2,3,4,5]
    Output: 5
    Explanation:
        No subarray with length greater than 1 is a palindrome.
        The largest element in the array is 5. Therefore, the answer is 5.

Example 5:
    Input: nums = [1000]
    Output: 1000
    Explanation:
        The subarray with only one element is a palindrome. Therefore, the answer is 1000.

Constraints:
    1 <= nums.length <= 105
    1 <= nums[i] <= 109
*/

impl Solution {
    // binary search + rolling hash
    pub fn get_sum(nums: Vec<i32>) -> i64 {
        const MOD1: i64 = 1_000_000_007;
        const MOD2: i64 = 1_000_000_009;
        const BASE: i64 = 911_382_323;

        fn build_hash(nums: &[i32], modu: i64) -> Vec<i64> {
            let mut h = vec![0i64; nums.len() + 1];
            for i in 0..nums.len() {
                let val = nums[i] as i64 % modu;
                h[i + 1] = (h[i] * BASE + val) % modu;
            }
            h
        }

        fn build_pow(n: usize, modu: i64) -> Vec<i64> {
            let mut p = vec![1i64; n + 1];
            for i in 0..n {
                p[i + 1] = p[i] * BASE % modu;
            }
            p
        }

        fn get_hash(h: &[i64], p: &[i64], l: usize, r: usize, modu: i64) -> i64 {
            let len = r - l + 1;
            (h[r + 1] - h[l] * p[len] % modu + modu) % modu
        }

        let n = nums.len();
        let mut prefix = vec![0i64; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + nums[i] as i64;
        }

        let range_sum = |l: usize, r: usize| -> i64 { prefix[r + 1] - prefix[l] };

        let rev: Vec<i32> = nums.iter().rev().copied().collect();

        let h1 = build_hash(&nums, MOD1);
        let h2 = build_hash(&nums, MOD2);
        let r1 = build_hash(&rev, MOD1);
        let r2 = build_hash(&rev, MOD2);

        let p1 = build_pow(n, MOD1);
        let p2 = build_pow(n, MOD2);

        let is_pal = |l: usize, r: usize| -> bool {
            let rl = n - 1 - r;
            let rr = n - 1 - l;
            get_hash(&h1, &p1, l, r, MOD1) == get_hash(&r1, &p1, rl, rr, MOD1)
                && get_hash(&h2, &p2, l, r, MOD2) == get_hash(&r2, &p2, rl, rr, MOD2)
        };

        let mut res = 0i64;

        for i in 0..n {
            // even
            let mut lo = 1usize;
            let mut hi = (i + 1).min(n - i);
            let mut best = 1usize;

            while lo <= hi {
                let mid = lo + ((hi - lo) >> 1);
                let l = i - mid + 1;
                let r = i + mid - 1;

                if is_pal(l, r) {
                    best = mid;
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            }

            let l = i - best + 1;
            let r = i + best - 1;
            res = res.max(range_sum(l, r));
        }

        for i in 1..n {
            // odd
            let mut lo = 1usize;
            let mut hi = i.min(n - i);
            let mut best = 0usize;

            while lo <= hi {
                let mid = lo + ((hi - lo) >> 1);
                let l = i - mid;
                let r = i + mid - 1;

                if is_pal(l, r) {
                    best = mid;
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            }

            if best > 0 {
                let l = i - best;
                let r = i + best - 1;
                res = res.max(range_sum(l, r));
            }
        }

        res
    }
}
