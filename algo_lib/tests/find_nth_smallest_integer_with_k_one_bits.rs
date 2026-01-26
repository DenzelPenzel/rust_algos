/*
You are given two positive integers n and k.

Return an integer denoting the nth smallest positive integer that 
has exactly k ones in its binary representation. 

It is guaranteed that the answer is strictly less than 250.

Example 1:
    Input: n = 4, k = 2
    Output: 9
    Explanation:
        The 4 smallest positive integers that have exactly k = 2 ones in their binary representations are:
        3 = 112
        5 = 1012
        6 = 1102
        9 = 10012

Example 2:
    Input: n = 3, k = 1
    Output: 4
    Explanation:
        The 3 smallest positive integers that have exactly k = 1 one in their binary representations are:
        1 = 12
        2 = 102
        4 = 1002
    
Constraints:
    1 <= n <= 250
    1 <= k <= 50
    The answer is strictly less than 250.
*/

struct Solution; 

impl Solution {
    pub fn nth_smallest(n: i64, k: i32) -> i64 {
        let mut lo = 1;
        let mut hi = 1u64 << 50;
        

        fn comb(n: u64, r: u64) -> u64 {
            if r > n {
                return 0;
            }
            if r == 0 || r == n {
                return 1;
            }
            if r > n / 2 {
                return comb(n, n - r);
            }
            let mut res = 1;
            for i in 1..=r {
                res = res * (n - i + 1) / i;
            }
            res
        }

        let count_le = |target: u64| -> u64 {
            let mut need = k as i64;
            let mut res = 0;
            let bit_len = 64 - target.leading_zeros();
            if bit_len == 0 {
                return if need == 0 { 1 } else { 0 };
            }
            for i in (0..bit_len).rev() {
                if (target >> i) & 1 == 1 {
                    if (i as i64) >= need {
                        res += comb(i as u64, need as u64);
                    }
                    need -= 1;
                }

                if need < 0 {
                    return res;
                }
            }
            if need == 0 {
                res += 1
            }
            res
        };

        let n_u64 = n as u64;
        let mut res: i64 = -1;

        while lo < hi {
            let mid = lo + ((hi - lo) >> 1);
            let cnt = count_le(mid);

            if cnt >= n_u64  {
                res = mid as i64;
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        res
    }
}