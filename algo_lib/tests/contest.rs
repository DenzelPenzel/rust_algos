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

use std::collections::{BTreeSet, HashSet};
use std::sync::atomic::Ordering;
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

use std::{cmp, i32, vec};

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


// impl Solution {
//     pub fn palindrome_path(n: i32, edges: Vec<Vec<i32>>, s: String, queries: Vec<String>) -> Vec<bool> {
//         let n = n as usize;
//         let mut graph = vec![vec![]; n];
        
//         for edge in edges {
//             let u = edge[0] as usize;
//             let v = edge[1] as usize;
//             graph[u].push(v);
//             graph[v].push(u);
//         }

//         let mut parent = vec![0; n];
//         let mut depth = vec![0; n];
//         let mut heavy_child = vec![None; n];
//         let mut size = vec![1; n];

//         Self::dfs(0, 0, 0, &graph, &mut parent, &mut depth, &mut size, &mut heavy_child);

//         let mut head = vec![0; n];
//         let mut pos = vec![0; n];
//         let mut cur_pos = 0;

//         Self::dfs_hld(0, 0, &graph, &parent, &mut head, &mut pos, &mut cur_pos, &heavy_child);


//         let mut tree = vec![0; 4 * n];
//         let chars = s.as_bytes();

//         for i in 0..n {
//             let val = 1 << (chars[i] - b'a');
//             Self::update_tree(1, 0, n - 1, pos[i], val, &mut tree);
//         }


//         let mut res: Vec<bool> = Vec::new();


//         for q in queries {
//             let mut parts = q.split_ascii_whitespace();
//             let type_str = parts.next().unwrap();
            
//             if type_str == "update" {
//                 let u = parts.next().unwrap().parse::<usize>().unwrap();
//                 let c_str = parts.next().unwrap();
//                 let c = c_str.as_bytes()[0];
//                 let val = 1 << (c - b'a');
//                 Self::update_tree(1, 0, n - 1, pos[u], val, &mut tree);
//             } else {
//                 let u = parts.next().unwrap().parse::<usize>().unwrap();
//                 let v = parts.next().unwrap().parse::<usize>().unwrap();
//                 let mask = Self::query_path(u, v, n, &parent, &depth, &head, &pos, &tree);
//                 if mask == 0 || (mask & (mask - 1)) == 0 {
//                     res.push(true);
//                 } else {
//                     res.push(false);
//                 }
//             }
//         }

//         res
//     }

//     fn dfs(
//         u: usize, 
//         p: usize, 
//         d: usize, 
//         graph: &Vec<Vec<usize>>, 
//         parent: &mut Vec<usize>, 
//         depth: &mut Vec<usize>, 
//         size: &mut Vec<usize>, 
//         heavy_child: &mut Vec<Option<usize>>,
//     ) {
//         parent[u] = p;
//         depth[u] = d;
//         let mut max_sz = 0;

//         for &v in &graph[u] {
//             if v != p {
//                 Self::dfs(v, u, d + 1, graph, parent, depth, size, heavy_child);
//                 size[u] += size[v];
//                 if size[v] > max_sz {
//                     max_sz = size[v];
//                     heavy_child[u] = Some(v);
//                 }
//             }
//         }
//     } 

//     fn dfs_hld(
//         u: usize,
//         h: usize,
//         graph: &Vec<Vec<usize>>,
//         parent: &Vec<usize>,
//         head: &mut Vec<usize>,
//         pos: &mut Vec<usize>,
//         cur_pos: &mut usize,
//         heavy_child: &Vec<Option<usize>>,
//     ) {
//         head[u] = h;
//         pos[u] = *cur_pos;
//         *cur_pos += 1;

//         if let Some(v) = heavy_child[u] {
//             Self::dfs_hld(v, h, graph, parent, head, pos, cur_pos, heavy_child)
//         }

//         for &v in &graph[u] {
//             if v != parent[u] && Some(v) != heavy_child[u] {
//                 Self::dfs_hld(v, v, graph, parent, head, pos, cur_pos, heavy_child);
//             }
//         }
//     }

//     fn update_tree(node: usize, start: usize, end: usize, idx: usize, val: i32, tree: &mut Vec<i32>) {
//         if start == end {
//             tree[node] = val;
//             return;
//         }

//         let mid = (start + end) / 2;
//         if idx <= mid {
//             Self::update_tree(2 * node, start, mid, idx, val, tree)
//         } else {
//             Self::update_tree(2 * node + 1, mid + 1, end, idx, val, tree)
//         }
//         tree[node] = tree[2 * node] ^ tree[2 * node + 1]
//     }

//     fn query_tree(node: usize, start: usize, end: usize, l: usize, r: usize, tree: &Vec<i32>) -> i32 {
//         if start > end {
//             return 0;
//         }
//         if r < start || end < l {
//             return 0;
//         }
//         if l <= start && end <= r {
//             return tree[node];
//         }
//         let mid = (start + end) / 2;
//         let p1 = Self::query_tree(node * 2, start, mid, l, r, tree);
//         let p2 =  Self::query_tree(node * 2 + 1, mid + 1, end, l, r, tree);
//         p1 ^ p2
//     }

//     fn query_path(
//         mut u: usize,
//         mut v: usize,
//         n: usize,
//         parent: &Vec<usize>,
//         depth: &Vec<usize>,
//         head: &Vec<usize>,
//         pos: &Vec<usize>,
//         tree: &Vec<i32>,
//     ) -> i32 {
//         let mut res = 0;
//         while head[u] != head[v] {
//             if depth[head[u]] > depth[head[v]] {
//                 res ^= Self::query_tree(1, 0, n - 1, pos[head[u]], pos[u], tree);
//                 u = parent[head[u]];
//             } else {
//                 res ^= Self::query_tree(1, 0, n - 1, pos[head[v]], pos[v], tree);
//                 v = parent[head[v]];
//             }
//         }

//         if depth[u] > depth[v] {
//             res ^= Self::query_tree(1, 0, n - 1, pos[v], pos[u], tree);
//         } else {
//             res ^= Self::query_tree(1, 0, n - 1, pos[u], pos[v], tree);
//         }
//         res
//     }
// }

// impl Solution {
//     pub fn min_distinct_freq_pair(nums: Vec<i32>) -> Vec<i32> {
//         let mut mp = HashMap::new();

//         for &x in &nums {
//             *mp.entry(x).or_insert(0) += 1;
//         }   

//         let mut unique: Vec<i32> = mp.keys().cloned().collect();
//         unique.sort_unstable();

//         for i in 0..unique.len() {
//             let x = unique[i];

//             for j in (i + 1)..unique.len() {
//                 let y = unique[j];

//                 if mp[&x] != mp[&y] {
//                     return vec![x, y];
//                 }
//             }
//         }

//         vec![-1, 1]
//     }
// }



// impl Solution {
//     pub fn merge_characters(s: String, k: i32) -> String {
//         let mut mp = HashMap::new();        
//         let mut seen: HashMap<char, usize> = HashMap::new();
//         let mut res: Vec<char> = Vec::with_capacity(s.len());
//         let k = k as usize;

//         for x in s.chars() {
//             if let Some(&prev_idx) = seen.get(&x) {
//                 let d = res.len() - prev_idx;

//                 if d <= k {
//                     continue;
//                 }
//             }

//             res.push(x);
//             seen.insert(x, res.len() - 1);
//         }

//         res.into_iter().collect()
//     }
// }

use std::{cmp, collections::HashMap};

// impl Solution {
//     pub fn make_parity_alternating(nums: Vec<i32>) -> Vec<i32> {
//         let n = nums.len();
//         if n == 0 || n == 1 {
//             return vec![0, 0];
//         }

//         let solve = |pos: i32| {
//             let mut ops = 0;
//             let mut range= Vec::new();

//             for (i, &val) in nums.iter().enumerate() {
//                 let rem = (pos + i as i32) % 2;
//                 if val.rem_euclid(2) == rem {
//                     range.push(vec![val]);
//                 } else {
//                     ops += 1;
//                     range.push(vec![val - 1, val + 1]);
//                 }
//             }

//             // [value, idx]
//             let mut lst = Vec::new();

//             for (idx, items) in range.iter().enumerate() {
//                 for &v in items {
//                     lst.push((v, idx));
//                 }
//             }

//             lst.sort_unstable();

//             let n = range.len();
//             let mut min_diff = i32::MAX;
//             let mut cnt = HashMap::new();
//             let mut lo = 0;

//             for hi in 0..lst.len() {
//                 *cnt.entry(lst[hi].1).or_insert(0) += 1;

//                 while cnt.len() == n {
//                     min_diff = min_diff.min(lst[hi].0 - lst[lo].0);
//                     let idx = lst[lo].1;
//                     if let Some(c) = cnt.get_mut(&idx) {
//                         *c -= 1;
//                         if *c == 0 {
//                             cnt.remove(&idx);
//                         }
//                     }
//                     lo += 1;
//                 }
//             }

//             (ops, min_diff)
//         };

//         let (ops0, min0) = solve(0);
//         let (ops1, min1) = solve(1);

//         if ops0 < ops1 {
//             vec![ops0, min0]
//         } else if ops1 < ops0 {
//             vec![ops1, min1]
//         } else {
//             vec![ops0, min0.min(min1)]
//         }
        
//     }
// }


// impl Solution {
//     pub fn sum_of_numbers(l: i32, r: i32, k: i32) -> i32 {
//         if k == 0 {
//             return 0;
//         }

//         let modulo: i64 = 1_000_000_007;

//         let sum: i64 = (l..=r).map(|x| x as i64).sum();
//         let m = (r - l + 1) as i64;
//         let k_64 = k as i64;

//         fn pow(mut base: i64, mut exp: i64, modulo: i64) -> i64 {
//             let mut res = 1;
//             base %= modulo;
//             while exp > 0 {
//                 if exp % 2 == 1 {
//                     res = (res * base) % modulo;
//                 }
//                 base = (base * base) % modulo;
//                 exp /= 2;
//             }
//             res
//         }

//         let m_pow = pow(m, k_64 - 1, modulo);
//         let t_pow = pow(10, k_64, modulo);
//         let inv9: i64 = 111_111_112;

//         let y = ((t_pow - 1 + modulo) % modulo * inv9) % modulo;

//         let mut res = (sum * m_pow) % modulo;
//         res = (res * y) % modulo;

//         res as i32
//     }
// }




// use std::{cmp, collections::HashMap};

// impl Solution {
//     pub fn make_parity_alternating(nums: Vec<i32>) -> Vec<i32> {
//         let n = nums.len();
//         if n == 0 || n == 1 {
//             return vec![0, 0];
//         }

//         let solve = |pos: i32| {
//             let mut pairs: Vec<_> = Vec::new();
//             let mut ops = 0;
            
//             for (i, &x) in nums.iter().enumerate() {
//                 let t = (pos + i as i32) % 2;
//                 if x.rem_euclid(2) == t {
//                     pairs.push((x, i));
//                 } else {
//                     pairs.push((x - 1, i));
//                     pairs.push((x + 1, i));
//                     ops += 1;
//                 }
//             }

//             // [val, idx] [val, idx]...

//             pairs.sort_unstable();

//             let mut lo = 0;
//             let mut hi = 0;
//             let mut seen = HashMap::new();
//             let mut min_diff = i32::MAX;

//             while hi < pairs.len() {
//                 let val_y = pairs[hi].0;
//                 let idx_y = pairs[hi].1;
//                 *seen.entry(idx_y).or_insert(0) += 1;

//                 while seen.len() == n {
//                     let val_x = pairs[lo].0;
//                     let idx_x = pairs[lo].1;
//                     min_diff = min_diff.min(val_y - val_x);
//                     if let Some(item) = seen.get_mut(&idx_x) {
//                         *item -= 1;
//                         if *item == 0 {
//                             seen.remove(&idx_x);
//                         }
//                     }
//                     lo += 1;
//                 }

//                 hi += 1;
//             }

//             (ops, min_diff)
//         };

//         let (ops0, min0) = solve(0);
//         let (ops1, min1) = solve(1);

//         if ops0 < ops1 {
//             vec![ops0, min0]
//         } else if ops1 < ops0 {
//             vec![ops1, min1]
//         } else {
//             vec![ops0, min0.min(min1)]
//         }
//     }
// }

use std::cmp::Ordering;

struct UnionFind {
    parent: Vec<usize>,
    weight: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            weight: vec![0; n],
            size: vec![1; n],
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }

        let mut st = Vec::new();
        while x != self.parent[x] {
            st.push(x);
            x = self.parent[x];
        }

        let root = x;
        let mut w = 0;

        while let Some(node) = st.pop() {
            self.parent[node] = root;
            self.weight[node] ^= w;
            w = self.weight[node];
        }
        x
    }

    pub fn union(&mut self, x: usize, y: usize, val: i32) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return (self.weight[x] ^ self.weight[y]) == val;
        }

        if self.size[root_x] > self.size[root_y] {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
            self.weight[root_y] = self.weight[x] ^ self. weight[y] ^ val;
        } else {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
            self.weight[root_x] = self.weight[x] ^ self. weight[y] ^ val;
        }

        true
    }
}

impl Solution {
    pub fn number_of_edges_added(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut uf = UnionFind::new(n as usize);
        let mut res = 0;

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[0] as usize;
            let w = edge[0];

            if uf.union(u, v, w) {
                res += 1;
            }
        }

        res
    }
}


struct UF {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UF {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }

        let mut st = Vec::new();
        while x != self.parent[x] {
            st.push(x);
            x = self.parent[x];
        }

        let root = x;

        while let Some(node) = st.pop() {
            self.parent[node] = root;
        }

        x
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false
        }

        if self.size[root_x] > self.size[root_y] {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        } else {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        }

        true
    }}

