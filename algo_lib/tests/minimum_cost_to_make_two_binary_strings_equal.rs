/*
You are given two binary strings s and t, both of length n, and three positive integers flipCost, swapCost, and crossCost.

You are allowed to apply the following operations any number of times (in any order) to the strings s and t:

Choose any index i and flip s[i] or t[i] (change '0' to '1' or '1' to '0'). The cost of this operation is flipCost.
Choose two distinct indices i and j, and swap either s[i] and s[j] or t[i] and t[j]. 
The cost of this operation is swapCost.

Choose an index i and swap s[i] with t[i]. The cost of this operation is crossCost.
Return an integer denoting the minimum total cost needed to make the strings s and t equal.

Example 1:
    Input: s = "01000", t = "10111", flipCost = 10, swapCost = 2, crossCost = 2
    Output: 16
    Explanation:
        We can perform the following operations:

        Swap s[0] and s[1] (swapCost = 2). After this operation, s = "10000" and t = "10111".
        Cross swap s[2] and t[2] (crossCost = 2). After this operation, s = "10100" and t = "10011".
        Swap s[2] and s[3] (swapCost = 2). After this operation, s = "10010" and t = "10011".
        Flip s[4] (flipCost = 10). After this operation, s = t = "10011".
        The total cost is 2 + 2 + 2 + 10 = 16.

Example 2:
    Input: s = "001", t = "110", flipCost = 2, swapCost = 100, crossCost = 100
    Output: 6
    Explanation:
        Flipping all the bits of s makes the strings equal, and the total cost is 3 * flipCost = 3 * 2 = 6.

Example 3:
    Input: s = "1010", t = "1010", flipCost = 5, swapCost = 5, crossCost = 5
    Output: 0
    Explanation:
        The strings are already equal, so no operations are required.

Constraints:
    n == s.length == t.length
    1 <= n <= 105​​​​​​​
    1 <= flipCost, swapCost, crossCost <= 109
    s and t consist only of the characters '0' and '1'.
*/

struct Solution;

impl Solution {
    pub fn minimum_cost(s: String, t: String, flip_cost: i32, swap_cost: i32, cross_cost: i32) -> i64 {
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();

        let mut count_a = 0;
        let mut count_b = 0;

        for i in 0..s.len() {
            if s_bytes[i] != t_bytes[i] {
                if s_bytes[i] == b'1' {
                    count_a += 1;
                } else {
                    count_b += 1;
                }
            }
        }

        let mut res: i64 = 0;

        let fc = flip_cost as i64;
        let sc = swap_cost as i64;
        let cc = cross_cost as i64;

        let d = std::cmp::min(count_a, count_b);
        res += d as i64 * std::cmp::min(fc * 2, sc);
        count_a -= d;
        count_b -= d;

        let c: i64 = count_a / 2;
    
        res += c as i64 * std::cmp::min(fc * 2, sc + cc);

        if count_a % 2 > 0 {
            res += fc;
        }
        
        let f = count_b / 2;

        res += f as i64 * std::cmp::min(fc * 2, sc + cc);
        if count_b % 2 > 0 {
            res += fc;
        }

        res
    }
}

#[test]
fn run_test() {
    assert_eq!(Solution::minimum_cost("01000".to_string(), "10111".to_string(), 10, 2, 2), 16);
}