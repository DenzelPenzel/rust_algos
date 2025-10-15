use std::collections::{HashSet, VecDeque};

struct Solution;

impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        let mut res = s.clone();
        let n = s.len();

        queue.push_back(s.clone());
        seen.insert(s);

        let add_val = a as u8;
        let rot_pos = (b % n as i32) as usize;

        while let Some(val) = queue.pop_front() {
            if val < res {
                res = val.clone();
            }
            let mut b = val.as_bytes().to_vec();
            for i in (1..n).step_by(2) {
                let digit = b[i] - b'0';
                b[i] = ((digit + add_val) % 10) + b'0';
            }

            let add_s = String::from_utf8(b).unwrap();
            if !seen.contains(&add_s) {
                seen.insert(add_s.clone());
                queue.push_back(add_s.clone());
            }

            let (end, start) = val.split_at(n - rot_pos);
            let rot_s = format!("{}{}", start, end);
            if !seen.contains(&rot_s) {
                seen.insert(rot_s.clone());
                queue.push_back(rot_s.clone());
            }
        }

        res
    }
}
