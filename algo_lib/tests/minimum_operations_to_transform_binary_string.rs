/*

You are given two binary strings s1 and s2 of the same length n.

You can perform the following operations on s1 any number of times, in any order:

Choose an index i such that s1[i] == '0', and change it to '1'.

Choose an index i such that 0 <= i < n - 1, and both s1[i] and s1[i + 1] are '1'.
Change both characters to '0'.

Return the minimum number of operations required to make s1 equal to s2.
If it is impossible, return -1.

Example 1:
    Input: s1 = "11", s2 = "00"
    Output: 1
    Explanation:
        Change indices 0 and 1 from '1' to '0' in one operation, so "11" becomes "00". Thus, the answer is 1.

Example 2:
    Input: s1 = "01", s2 = "10"
    Output: 3
    Explanation:
        Change index 0 from '0' to '1', so "01" becomes "11".
        Change indices 0 and 1 from '1' to '0', so "11" becomes "00".
        Change index 0 from '0' to '1', so "00" becomes "10".
        Thus, the answer is 3.

Example 3:
    Input: s1 = "1", s2 = "0"
    Output: -1
    Explanation:
        The first operation cannot change '1' to '0', and the second operation
        requires two adjacent characters. Therefore, it is impossible.

Constraints:
    1 <= n == s1.length == s2.length <= 105
    s1 and s2 consist only of '0' and '1'.
*/

impl Solution {
    pub fn min_operations(s1: String, s2: String) -> i32 {
        fn dfs(i: isize, used_prev: usize, a: &[u8], b: &[u8], dp: &mut Vec<[i32; 2]>) -> i32 {
            let inf = 1_000_000_000;

            if i < 0 {
                return if used_prev == 0 { 0 } else { inf };
            }

            let idx = i as usize;

            if dp[idx][used_prev] != -1 {
                return dp[idx][used_prev];
            }

            let mut res = inf;

            for use_cur in 0..=1usize {
                if idx == 0 && use_cur == 1 {
                    continue;
                }

                let cnt = used_prev + use_cur;
                let mut cost = 0;

                if cnt > 0 {
                    cost += (cnt - 1) as i32;

                    if a[idx] == b'0' {
                        cost += 1;
                    }

                    if b[idx] == b'1' {
                        cost += 1
                    }
                } else {
                    if a[idx] == b'1' && b[idx] == b'0' {
                        continue;
                    }

                    if a[idx] == b'0' && b[idx] == b'1' {
                        cost += 1;
                    }
                }

                if use_cur == 1 {
                    cost += 1
                }

                res = res.min(dfs(i - 1, use_cur, a, b, dp) + cost);
            }

            dp[idx][used_prev] = res;
            res
        }

        let a = s1.as_bytes();
        let b = s2.as_bytes();
        let n = a.len();
        let mut dp = vec![[-1; 2]; n];
        let res = dfs(n as isize - 1, 0, a, b, &mut dp);

        if res >= 1_000_000_000 {
            -1
        } else {
            res
        }
    }
}

impl Solution_II {
    pub fn min_operations(s1: String, s2: String) -> i32 {
        fn one_pos_cost(s1: u8, s2: u8, cnt: usize) -> i32 {
            let inf = 1_000_000_000;
            if cnt == 0 {
                return match (s1, s2) {
                    (b'0', b'0') => 0,
                    (b'0', b'1') => 1,
                    (b'1', b'0') => inf,
                    (b'1', b'1') => 0,
                    _ => inf,
                };
            }

            let mut cost = 0;
            if s1 == b'0' {
                cost += 1;
            }

            cost += (cnt - 1) as i32;

            if s2 == b'1' {
                cost += 1
            }

            cost
        }

        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let n = s1.len();
        let inf = 1_000_000_000;
        let mut dp = vec![vec![inf; 2]; n + 1];
        dp[0][0] = 0;

        for i in 0..n {
            for used_prev in 0..=1 {
                if dp[i][used_prev] >= inf {
                    continue;
                }

                for use_cur in 0..=1 {
                    if i + 1 == n && use_cur == 1 {
                        continue;
                    }

                    let cnt = used_prev + use_cur;
                    let cost = one_pos_cost(s1[i], s2[i], cnt);

                    if cost >= inf {
                        continue;
                    }

                    let total = dp[i][used_prev] + cost + use_cur as i32;
                    dp[i + 1][use_cur] = dp[i + 1][use_cur].min(total);
                }
            }
        }

        if dp[n][0] >= inf {
            -1
        } else {
            dp[n][0]
        }
    }
}
