/*
You are given three strings word1, word2, and target.

Your task is to count the number of ways to form target by choosing characters from word1
and word2 under the following conditions:

For each character of target, choose one matching character from either word1 or word2.
The chosen indices from word1 must be strictly increasing.
The chosen indices from word2 must be strictly increasing.
At least one character must be chosen from both word1 and word2.
Two ways are considered different if, for at least one position in target, the chosen
character comes from a different string or a different index.

Return the number of ways. Since the answer may be very large, return it modulo 109 + 7.

Example 1:
    Input: word1 = "abc", word2 = "bac", target = "abc"
    Output: 5
    Explanation:
        There are 5 ways to form target:

        word1[0] = 'a', word1[1] = 'b', word2[2] = 'c'
        word1[0] = 'a', word2[0] = 'b', word1[2] = 'c'
        word1[0] = 'a', word2[0] = 'b', word2[2] = 'c'
        word2[1] = 'a', word1[1] = 'b', word1[2] = 'c'
        word2[1] = 'a', word1[1] = 'b', word2[2] = 'c'
        All ways preserve the increasing index order inside each string and
        choose at least one character from each string.

Example 2:
    Input: word1 = "cd", word2 = "cd", target = "ccd"
    Output: 4
    Explanation:
        There are 4 ways to form target:
        word1[0] = 'c', word2[0] = 'c', word1[1] = 'd'
        word1[0] = 'c', word2[0] = 'c', word2[1] = 'd'
        word2[0] = 'c', word1[0] = 'c', word1[1] = 'd'
        word2[0] = 'c', word1[0] = 'c', word2[1] = 'd'
        The first two 'c' characters in target must come one from each string.
        The final 'd' can be chosen from either string.

Example 3:
    Input: word1 = "xy", word2 = "xy", target = "xyxy"
    Output: 2
    Explanation:
        There are 2 ways to form target:
        word1[0] = 'x', word1[1] = 'y', word2[0] = 'x', word2[1] = 'y'
        word2[0] = 'x', word2[1] = 'y', word1[0] = 'x', word1[1] = 'y'
        Each "xy" part in target comes entirely from one string.

Example 4:
    Input: word1 = "ab", word2 = "cde", target = "ace"
    Output: 1
    Explanation:
        The only way is to choose word1[0] = 'a', word2[0] = 'c', and word2[2] = 'e'. Thus, the answer is 1.

Constraints:
    1 <= word1.length, word2.length, target.length <= 100
    word1, word2, and target consist of lowercase English letters only.
*/

use std::collections::HashMap;

impl Solution {
    pub fn interleave_characters(word1: String, word2: String, target: String) -> i32 {
        const MOD: i64 = 1_000_000_007;

        type Key = (usize, usize, usize, usize);

        fn dfs(
            i: usize,
            p1: usize,
            p2: usize,
            mask: usize,
            w1: &[u8],
            w2: &[u8],
            target: &[u8],
            memo: &mut HashMap<Key, i64>,
            memo1: &mut HashMap<Key, i64>,
            memo2: &mut HashMap<Key, i64>,
        ) -> i64 {
            if i == target.len() {
                return if mask == 3 { 1 } else { 0 };
            }

            let key = (i, p1, p2, mask);

            if let Some(&val) = memo.get(&key) {
                return val;
            }

            let res = (sum1(i, p1, p2, mask, w1, w2, target, memo, memo1, memo2)
                + sum2(i, p1, p2, mask, w1, w2, target, memo, memo1, memo2))
                % MOD;

            memo.insert(key, res);

            res
        }

        fn sum1(
            i: usize,
            p1: usize,
            p2: usize,
            mask: usize,
            w1: &[u8],
            w2: &[u8],
            target: &[u8],
            memo: &mut HashMap<Key, i64>,
            memo1: &mut HashMap<Key, i64>,
            memo2: &mut HashMap<Key, i64>,
        ) -> i64 {
            if p1 == w1.len() {
                return 0;
            }

            let key = (i, p1, p2, mask);

            if let Some(&val) = memo1.get(&key) {
                return val;
            }

            let mut res = sum1(i, p1 + 1, p2, mask, w1, w2, target, memo, memo1, memo2);

            if w1[p1] == target[i] {
                res += dfs(
                    i + 1,
                    p1 + 1,
                    p2,
                    mask | 1,
                    w1,
                    w2,
                    target,
                    memo,
                    memo1,
                    memo2,
                );
                res %= MOD;
            }

            memo1.insert(key, res);
            res
        }

        fn sum2(
            i: usize,
            p1: usize,
            p2: usize,
            mask: usize,
            w1: &[u8],
            w2: &[u8],
            target: &[u8],
            memo: &mut HashMap<Key, i64>,
            memo1: &mut HashMap<Key, i64>,
            memo2: &mut HashMap<Key, i64>,
        ) -> i64 {
            if p2 == w2.len() {
                return 0;
            }

            let key = (i, p1, p2, mask);

            if let Some(&val) = memo2.get(&key) {
                return val;
            }

            let mut res = sum2(i, p1, p2 + 1, mask, w1, w2, target, memo, memo1, memo2);

            if w2[p2] == target[i] {
                res += dfs(
                    i + 1,
                    p1,
                    p2 + 1,
                    mask | 2,
                    w1,
                    w2,
                    target,
                    memo,
                    memo1,
                    memo2,
                );
                res %= MOD;
            }

            memo2.insert(key, res);
            res
        }

        let w1 = word1.into_bytes();
        let w2 = word2.into_bytes();
        let target = target.into_bytes();
        let mut memo = HashMap::new();
        let mut memo1 = HashMap::new();
        let mut memo2 = HashMap::new();

        dfs(
            0, 0, 0, 0, &w1, &w2, &target, &mut memo, &mut memo1, &mut memo2,
        ) as i32
    }
}
