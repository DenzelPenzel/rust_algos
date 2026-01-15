/*
You are given a string array words, consisting of distinct 4-letter strings, each containing lowercase English letters.

A word square consists of 4 distinct words: top, left, right and bottom, arranged as follows:

top forms the top row.
bottom forms the bottom row.
left forms the left column (top to bottom).
right forms the right column (top to bottom).
It must satisfy:

top[0] == left[0], top[3] == right[0]
bottom[0] == left[3], bottom[3] == right[3]
Return all valid distinct word squares, sorted in ascending lexicographic order by the 4-tuple (top, left, right, bottom)​​​​​​​.

Example 1:
    Input: words = ["able","area","echo","also"]
    Output: [["able","area","echo","also"],["area","able","also","echo"]]
    Explanation:
        There are exactly two valid 4-word squares that satisfy all corner constraints:

        "able" (top), "area" (left), "echo" (right), "also" (bottom)
        top[0] == left[0] == 'a'
        top[3] == right[0] == 'e'
        bottom[0] == left[3] == 'a'
        bottom[3] == right[3] == 'o'
        "area" (top), "able" (left), "also" (right), "echo" (bottom)
        All corner constraints are satisfied.
        Thus, the answer is [["able","area","echo","also"],["area","able","also","echo"]].

Example 2:
    Input: words = ["code","cafe","eden","edge"]
    Output: []
    Explanation:
        No combination of four words satisfies all four corner constraints. Thus, the answer is empty array [].

Constraints:
    4 <= words.length <= 15
    words[i].length == 4
    words[i] consists of only lowercase English letters.
    All words[i] are distinct.
*/


struct Solution;

impl Solution {
    pub fn word_squares(words: Vec<String>) -> Vec<Vec<String>> {
        let mut res = Vec::new();

        let mut refs: Vec<&String> = words.iter().collect();

        let k = 4;
        Self::permute(0, k, &mut refs, &mut res);
        
        res.sort();
        res 
    }

    fn permute(i: usize, k: usize, items: &mut Vec<&String>, res: &mut Vec<Vec<String>>) {
       if i == k {
            let cand = &items[0..k];
            let t = cand[0].as_bytes();
            let l = cand[1].as_bytes();
            let r = cand[2].as_bytes();
            let b = cand[3].as_bytes();

            if t[0] == l[0] && t[3] == r[0] && b[0] == l[3] && b[3] == r[3] {
                res.push(cand.iter().map(|s| s.to_string()).collect());
            }

            return;
       }

       for ii in i..items.len() {
        items.swap(i, ii);
        Self::permute(i + 1, k, items, res);
        items.swap(i, ii);
       }
    }
}

#[test]
fn test() {
    let words = vec!["able", "area", "echo", "also"].iter().map(|s| s.to_string()).collect();
    let res = Solution::word_squares(words);
    println!("{:?}", res);
}