// impl Solution {
//     pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
//         words.iter().map(|word| {
//             let sum: i32 = word.bytes().map(|b| {
//                 let idx = (b - b'a') as usize;
//                 weights[idx]
//             }).sum();

//             let rem = sum % 26;
//             let mapped_byte = b'a' + (25 - rem as u8);
//             mapped_byte as char
//         }).collect()
//     }
// }

use std::{cmp, collections::HashMap};

// impl Solution {
//     pub fn prefix_connected(words: Vec<String>, k: i32) -> i32 {
//         let mut mapping = HashMap::new();
//         let k = k as usize;

//         for word in &words {
//             if word.len() < k {
//                 continue;
//             }

//             let prefix = &word[..k];

//             *mapping.entry(prefix).or_insert(0) += 1;
//         }

//         let mut res = 0;

//         for &count in mapping.values() {
//             if count >= 2 {
//                 res += 1;
//             }
//         }

//         res
//     }
// }

use std::cmp;

// impl Solution {
//     pub fn rob(nums: Vec<i32>, colors: Vec<i32>) -> i64 {
//         let n = nums.len();
//         if n == 0 {
//             return 0;
//         }
//         let mut memo = vec![None; n];

//         Self::solve(n - 1, nums, colors, memo)
//     }

//     fn solve(i: usize, nums: &Vec<i32>, colors: &Vec<i32> , memo: &mut Vec<Option<i64>>) -> i64 {
//         if i == 0 {
//             return nums[0] as i64;
//         }

//         if let Some(val) = memo[i] {
//             return val;
//         }

//         let curr = nums[i] as i64;

//         let res = if colors[i] != colors[i - 1] {
//             Self::solve(i - 1, nums, colors, memo) + curr
//         } else {
//             let skip_current = Self::solve(i - 1, nums, colors, memo);

//             let take_current = if i >= 2 {
//                 Self::solve(i - 2, nums, colors, memo) + curr
//             } else {
//                 curr
//             };

//             cmp::max(skip_current, take_current)
//         };

//         memo[i] = Some(res);

//         return res;
//     }
// }


impl Solution {
    pub fn palindrome_path(n: i32, edges: Vec<Vec<i32>>, s: String, queries: Vec<String>) -> Vec<bool> {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }

        let mut parent = vec![0; n];
        let mut depth = vec![0; n];
        let mut heavy_child = vec![None; n];
        let mut size = vec![1; n];

        Self::dfs(0, 0, 0, &graph, &mut parent, &mut depth, &mut size, &mut heavy_child);

        let mut head = vec![0; n];
        let mut pos = vec![0; n];
        let mut cur_pos = 0;

        Self::dfs_hld(0, 0, &graph, &parent, &mut head, &mut pos, &mut cur_pos, &heavy_child);


        let mut tree = vec![0; 4 * n];
        let chars = s.as_bytes();

        for i in 0..n {
            let val = 1 << (chars[i] - b'a');
            Self::update_tree(1, 0, n - 1, pos[i], val, &mut tree);
        }


        let mut res: Vec<bool> = Vec::new();


        for q in queries {
            let mut parts = q.split_ascii_whitespace();
            let type_str = parts.next().unwrap();
            
            if type_str == "update" {
                let u = parts.next().unwrap().parse::<usize>().unwrap();
                let c_str = parts.next().unwrap();
                let c = c_str.as_bytes()[0];
                let val = 1 << (c - b'a');
                Self::update_tree(1, 0, n - 1, pos[u], val, &mut tree);
            } else {
                let u = parts.next().unwrap().parse::<usize>().unwrap();
                let v = parts.next().unwrap().parse::<usize>().unwrap();
                let mask = Self::query_path(u, v, n, &parent, &depth, &head, &pos, &tree);
                if mask == 0 || (mask & (mask - 1)) == 0 {
                    res.push(true);
                } else {
                    res.push(false);
                }
            }
        }

        res
    }

    fn dfs(
        u: usize, 
        p: usize, 
        d: usize, 
        graph: &Vec<Vec<usize>>, 
        parent: &mut Vec<usize>, 
        depth: &mut Vec<usize>, 
        size: &mut Vec<usize>, 
        heavy_child: &mut Vec<Option<usize>>,
    ) {
        parent[u] = p;
        depth[u] = d;
        let mut max_sz = 0;

        for &v in &graph[u] {
            if v != p {
                Self::dfs(v, u, d + 1, graph, parent, depth, size, heavy_child);
                size[u] += size[v];
                if size[v] > max_sz {
                    max_sz = size[v];
                    heavy_child[u] = Some(v);
                }
            }
        }
    } 

    fn dfs_hld(
        u: usize,
        h: usize,
        graph: &Vec<Vec<usize>>,
        parent: &Vec<usize>,
        head: &mut Vec<usize>,
        pos: &mut Vec<usize>,
        cur_pos: &mut usize,
        heavy_child: &Vec<Option<usize>>,
    ) {
        head[u] = h;
        pos[u] = *cur_pos;
        *cur_pos += 1;

        if let Some(v) = heavy_child[u] {
            Self::dfs_hld(v, h, graph, parent, head, pos, cur_pos, heavy_child)
        }

        for &v in &graph[u] {
            if v != parent[u] && Some(v) != heavy_child[u] {
                Self::dfs_hld(v, v, graph, parent, head, pos, cur_pos, heavy_child);
            }
        }
    }

    fn update_tree(node: usize, start: usize, end: usize, idx: usize, val: i32, tree: &mut Vec<i32>) {
        if start == end {
            tree[node] = val;
            return;
        }

        let mid = (start + end) / 2;
        if idx <= mid {
            Self::update_tree(2 * node, start, mid, idx, val, tree)
        } else {
            Self::update_tree(2 * node + 1, mid + 1, end, idx, val, tree)
        }
        tree[node] = tree[2 * node] ^ tree[2 * node + 1]
    }

    fn query_tree(node: usize, start: usize, end: usize, l: usize, r: usize, tree: &Vec<i32>) -> i32 {
        if start > end {
            return 0;
        }
        if r < start || end < l {
            return 0;
        }
        if l <= start && end <= r {
            return tree[node];
        }
        let mid = (start + end) / 2;
        let p1 = Self::query_tree(node * 2, start, mid, l, r, tree);
        let p2 =  Self::query_tree(node * 2 + 1, mid + 1, end, l, r, tree);
        p1 ^ p2
    }

    fn query_path(
        mut u: usize,
        mut v: usize,
        n: usize,
        parent: &Vec<usize>,
        depth: &Vec<usize>,
        head: &Vec<usize>,
        pos: &Vec<usize>,
        tree: &Vec<i32>,
    ) -> i32 {
        let mut res = 0;
        while head[u] != head[v] {
            if depth[head[u]] > depth[head[v]] {
                res ^= Self::query_tree(1, 0, n - 1, pos[head[u]], pos[u], tree);
                u = parent[head[u]];
            } else {
                res ^= Self::query_tree(1, 0, n - 1, pos[head[v]], pos[v], tree);
                v = parent[head[v]];
            }
        }

        if depth[u] > depth[v] {
            res ^= Self::query_tree(1, 0, n - 1, pos[v], pos[u], tree);
        } else {
            res ^= Self::query_tree(1, 0, n - 1, pos[u], pos[v], tree);
        }
        res
    }
}