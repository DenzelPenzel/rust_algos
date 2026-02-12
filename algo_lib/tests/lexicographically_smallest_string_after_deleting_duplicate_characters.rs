/*
You are given a string s that consists of lowercase English letters.

You can perform the following operation any number of times (possibly zero times):

Choose any letter that appears at least twice in the current string s and delete any one occurrence.
Return the lexicographically smallest resulting string that can be formed this way.

Example 1:
    Input: s = "aaccb"
    Output: "aacb"
    Explanation:
        We can form the strings "acb", "aacb", "accb", and "aaccb". "aacb" is the lexicographically smallest one.
        For example, we can obtain "aacb" by choosing 'c' and deleting its first occurrence.

Example 2:
    Input: s = "z"
    Output: "z"
    Explanation:
        We cannot perform any operations. The only string we can form is "z".

Constraints:
    1 <= s.length <= 105
    s contains lowercase English letters only.
*/

struct Solution;

impl Solution {
    pub fn lex_smallest_after_deletion(s: String) -> String {
        let s_bytes = s.as_bytes();
        let mut st = Vec::new();
        let mut freq = vec![0; 26];

        for &b in s_bytes {
            freq[(b - b'a') as usize] += 1
        }

        let mut in_stack = [0; 26];

        for &b in s_bytes {
            freq[(b - b'a') as usize] -= 1;

            while let Some(&top) = st.last() {
                let top_idx = (top - b'a') as usize;

                if top > b && (in_stack[top_idx] + freq[top_idx] > 1) {
                    st.pop();
                    in_stack[top_idx] -= 1;
                } else {
                    break;
                }
            }

            st.push(b);
            in_stack[(b - b'a') as usize] += 1;
        }

        while let Some(&top) = st.last() {
            let top_idx = (top - b'a') as usize;
            if in_stack[top_idx] > 1 {
                st.pop();
                in_stack[top_idx] -= 1;
            } else {
                break;
            }
        }

        String::from_utf8(st).unwrap()
    }
}
