/*
You are given an array words of n strings. Each string has length m and contains only lowercase English letters.

Two strings s and t are similar if we can apply the following operation any number of times (possibly zero times) so that s and t become equal.

Choose either s or t.
Replace every letter in the chosen string with the next letter in the alphabet cyclically. The next letter after 'z' is 'a'.
Count the number of pairs of indices (i, j) such that: i < j words[i] and words[j] are similar.
Return an integer denoting the number of such pairs.

Example 1:
    Input: words = ["fusion","layout"]
    Output: 1
    Explanation:
        words[0] = "fusion" and words[1] = "layout" are similar because we can apply the operation to "fusion" 6 times. The string "fusion" changes as follows.

        "fusion"
        "gvtjpo"
        "hwukqp"
        "ixvlrq"
        "jywmsr"
        "kzxnts"
        "layout"

Example 2:
    Input: words = ["ab","aa","za","aa"]
    Output: 2
    Explanation:
        words[0] = "ab" and words[2] = "za" are similar. words[1] = "aa" and words[3] = "aa" are similar.

Constraints:
    1 <= n == words.length <= 105
    1 <= m == words[i].length <= 105
    1 <= n * m <= 105
    words[i] consists only of lowercase English letters.
*/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn count_pairs(words: Vec<String>) -> i64 {
        if words.len() == 1 {
            return 0
        }

        fn get_key(word: &str) -> Vec<u8> {
            if word.len() == 1 {
                return Vec::new()
            }

            let bytes = word.as_bytes();
            let mut res = Vec::with_capacity(word.len() - 1);

            for w in bytes.windows(2) {
                let a = w[0];
                let b = w[1];
                let diff = (b as i16 - a as i16 + 26) % 26;
                res.push(diff as u8);
            } 

            res
        }

        let mut counter: HashMap<Vec<u8>, i64> = HashMap::new();
        for word in words {
            let key = get_key(&word);
            *counter.entry(key).or_insert(0) += 1;
        }

        let mut res = 0;
        for &freq in counter.values() {
            res += freq * (freq - 1) / 2;
        }
        
        res
    }
}


#[test]
fn run_test() {
    let words = vec!["fusion", "layout"].into_iter().map(|s| s.to_string()).collect();
    assert_eq!(Solution::count_pairs(words), 1);

    let words = vec!["ab","aa","za","aa"].into_iter().map(|s| s.to_string()).collect();
    assert_eq!(Solution::count_pairs(words), 2);
}