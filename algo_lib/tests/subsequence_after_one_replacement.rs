/*
You are given two strings s and t consisting of lowercase English letters.

You may choose at most one index in s and replace the character at that index
with any lowercase English letter.

Return true if it is possible to make s a subsequence of t; otherwise, return false.

Example 1:
    Input: s = "cat", t = "chat"
    Output: true
    Explanation:
        Replace s[1] from 'a' to 'h'. The resulting string is "cht".
        "cht" is a subsequence of "chat" because we can match 'c', 'h', and 't' in order.

Example 2:
    Input: s = "plane", t = "apple"
    Output: false
    Explanation:
        The characters 'p', 'l', and 'e' can be matched in t, but the remaining
        characters cannot be matched while preserving the required order.
        Even after replacing any one character in s, it is impossible to make s a subsequence of t.

Constraints:
    1 <= s.length, t.length <= 105
    s and t consist only of lowercase English letters.
*/

impl Solution {
    pub fn can_make_subsequence(s: String, t: String) -> bool {
        let s = s.into_bytes();
        let t = t.into_bytes();

        if s.len() > t.len() {
            return false;
        }

        if s.is_empty() {
            return true;
        }

        let n = t.len();
        let not_used = 0usize;
        let used = 1usize;

        let mut dp = vec![[0usize; 2]; n];

        dp[0][not_used] = 0;
        dp[0][used] = 1;

        if s[0] == t[0] {
            dp[0][not_used] = 1;
        }

        for i in 1..n {
            dp[i][not_used] = dp[i - 1][not_used];
            dp[i][used] = dp[i - 1][used];

            let prev_used = dp[i - 1][used];
            if prev_used < s.len() && s[prev_used] == t[i] {
                dp[i][used] = dp[i][used].max(prev_used + 1);
            }

            let prev_not_used = dp[i - 1][not_used];
            if prev_not_used < s.len() && s[prev_not_used] == t[i] {
                dp[i][not_used] = dp[i][not_used].max(prev_not_used + 1);
            }

            if prev_not_used < s.len() {
                dp[i][used] = dp[i][used].max(prev_not_used + 1)
            }

            if dp[i][not_used] == s.len() || dp[i][used] == s.len() {
                return true;
            }
        }

        dp[n - 1][not_used] == s.len() || dp[n - 1][used] == s.len()
    }
}

impl Solution_II {
    pub fn can_make_subsequence(s: String, t: String) -> bool {
        fn dfs(
            i: usize,
            j: usize,
            used: usize,
            s: &[u8],
            t: &[u8],
            dp: &mut Vec<Vec<[i8; 2]>>,
        ) -> bool {
            if i == s.len() {
                return true;
            }

            if j == t.len() {
                return false;
            }

            if dp[i][j][used] != -1 {
                return dp[i][j][used] == 1;
            }

            let mut ok = false;
            if dfs(i, j + 1, used, s, t, dp) {
                ok = true;
            }

            if s[i] == t[j] && dfs(i + 1, j + 1, used, s, t, dp) {
                ok = true
            }

            if used == 0 && dfs(i + 1, j, 1, s, t, dp) {
                ok = true
            }

            dp[i][j][used] = if ok { 1 } else { 0 };

            ok
        }

        let s = s.into_bytes();
        let t = t.into_bytes();

        let mut dp = vec![vec![[-1; 2]; t.len() + 1]; s.len() + 1];
        dfs(0, 0, 0, &s, &t, &mut dp);
    }
}

impl Solution {
    pub fn max_digit_range(nums: Vec<i32>) -> i32 {
        fn digit_range(mut x: i32) -> i32 {
            let mut mn = 9;
            let mut mx = 0;

            while x > 0 {
                let d = x % 10;
                mn = mn.min(d);
                mx = mx.max(d);
                x = x / 10;
            }
            mx - mn
        }

        let mut best = -1;
        let mut res = 0;

        for num in nums {
            let x = digit_range(num);
            if x > best {
                best = x;
                res = num;
            } else if x == best {
                res += num;
            }
        }

        res
    }
}
