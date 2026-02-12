/*
You are given two 0-indexed strings source and target, 
both of length n and consisting of lowercase English letters. 

You are also given two 0-indexed character arrays original and changed, 
and an integer array cost, where cost[i] represents 
the cost of changing the character original[i] to 
the character changed[i].

You start with the string source. 
In one operation, you can pick a character x from 
the string and change it to the character y at a cost of z 
if there exists any index j such that 
cost[j] == z, original[j] == x, and changed[j] == y.

Return the minimum cost to convert the string source 
to the string target using any number of operations. 

If it is impossible to convert source to target, return -1.

Note that there may exist indices i, j such that 
original[j] == original[i] and changed[j] == changed[i].

Example 1:
    Input: source = "abcd", target = "acbe", original = ["a","b","c","c","e","d"], changed = ["b","c","b","e","b","e"], cost = [2,5,5,1,2,20]
    Output: 28
    Explanation: To convert the string "abcd" to string "acbe":
    - Change value at index 1 from 'b' to 'c' at a cost of 5.
    - Change value at index 2 from 'c' to 'e' at a cost of 1.
    - Change value at index 2 from 'e' to 'b' at a cost of 2.
    - Change value at index 3 from 'd' to 'e' at a cost of 20.
    The total cost incurred is 5 + 1 + 2 + 20 = 28.
    It can be shown that this is the minimum possible cost.

Example 2:
    Input: source = "aaaa", target = "bbbb", original = ["a","c"], changed = ["c","b"], cost = [1,2]
    Output: 12
    Explanation: To change the character 'a' to 'b' change the character 'a' to 'c' at a cost of 1, followed by changing the character 'c' to 'b' at a cost of 2, for a total cost of 1 + 2 = 3. To change all occurrences of 'a' to 'b', a total cost of 3 * 4 = 12 is incurred.

Example 3:
    Input: source = "abcd", target = "abce", original = ["a"], changed = ["e"], cost = [10000]
    Output: -1
    Explanation: It is impossible to convert source to target because the value at index 3 cannot be changed from 'd' to 'e'.

Constraints:
    1 <= source.length == target.length <= 105
    source, target consist of lowercase English letters.
    1 <= cost.length == original.length == changed.length <= 2000
    original[i], changed[i] are lowercase English letters.
    1 <= cost[i] <= 106
    original[i] != changed[i]
*/

use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}, i64};

struct Solution;


impl Solution {
    pub fn minimum_cost(source: String, target: String, original: Vec<char>, changed: Vec<char>, cost: Vec<i32>) -> i64 {
        let mut res = 0;
        let mut graph: HashMap<char, Vec<(char, i64)>> = HashMap::new();

        for (i, &u) in original.iter().enumerate() {
            let v = changed[i];
            let c = cost[i] as i64;
            graph.entry(u).or_default().push((v, c));
        }

        let mut memo: HashMap<(char, char), i64> = HashMap::new();

        let s_chars: Vec<char> = source.chars().collect();
        let t_chars: Vec<char> = target.chars().collect();

        for (i, &a) in s_chars.iter().enumerate() {
            let b = t_chars[i];
            if a != b {
                if let Some(&val) = memo.get(&(a, b)) {
                    res += val;
                } else {
                    let x = Self::dijkstra(a, b, &graph, &mut memo);
                    if x == -1 {
                        return -1;
                    }
                    res += x;
                }
            }
        }

        res
    }

    fn dijkstra(a: char, b: char, graph: &HashMap<char, Vec<(char, i64)>>, memo: &mut HashMap<(char, char), i64>) -> i64 {
        let mut queue = BinaryHeap::new();
        queue.push((Reverse(0), a));

        let mut dist: HashMap<char, i64> = HashMap::new();
        dist.insert(a, 0);

        while let Some((Reverse(c), x)) = queue.pop() {
            if x == b {
                memo.insert((a, b), c);
                return c;
            }

            if c > *dist.get(&x).unwrap_or(&i64::MAX) {
                continue;
            }

            if let Some(nei) = graph.get(&x) {
                for &(xx, cc) in nei {
                    let dist_xx = *dist.get(&xx).unwrap_or(&i64::MAX);
                    if dist_xx > c + cc {
                        dist.insert(xx, c + cc);
                        queue.push((Reverse(c + cc), xx));
                    }
                }
            }
        }

        -1
    }
}