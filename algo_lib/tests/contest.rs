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

use std::cmp::{max, Ordering, Reverse};
use std::collections::{BTreeSet, BinaryHeap, HashSet, VecDeque};
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

use std::{cmp, i32, i64, mem, vec};

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
            self.weight[root_y] = self.weight[x] ^ self.weight[y] ^ val;
        } else {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
            self.weight[root_x] = self.weight[x] ^ self.weight[y] ^ val;
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
            return false;
        }

        if self.size[root_x] > self.size[root_y] {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        } else {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        }

        true
    }
}

impl Solution {
    pub fn filter_occupied_intervals(
        mut occupied_intervals: Vec<Vec<i32>>,
        free_start: i32,
        free_end: i32,
    ) -> Vec<Vec<i32>> {
        if occupied_intervals.is_empty() {
            return vec![];
        }

        occupied_intervals.sort_by_key(|v| v[0]);

        let mut merged: Vec<(i64, i64)> = Vec::new();

        for interval in occupied_intervals {
            let s = interval[0] as i64;
            let e = interval[1] as i64;

            if let Some(prev) = merged.last_mut() {
                if prev.1 < s {
                    merged.push((s, e));
                } else {
                    prev.1 = prev.1.max(e);
                }
            } else {
                merged.push((s, e));
            }
        }

        let free_start = free_start as i64;
        let free_end = free_end as i64;
        let mut ans = Vec::new();

        for (s, e) in merged {
            if e < free_start || s > free_end {
                ans.push(vec![s as i32, e as i32]);
            } else {
                if s < free_start {
                    ans.push(vec![s as i32, (free_start - 1) as i32]);
                }

                if e > free_end {
                    ans.push(vec![(free_end + 1) as i32, e as i32]);
                }
            }
        }

        ans
    }
}

impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32, mul: i32) -> i64 {
        let mut pos = Vec::new();
        let mut neg = Vec::new();
        let mut res = 0i64;
        let mut zeros = 0;

        for num in nums {
            if num > 0 {
                pos.push(num as i64);
            } else if num < 0 {
                neg.push(num as i64);
            } else {
                zeros += 1;
            }
        }

        pos.sort_unstable_by(|a, b| b.cmp(a));
        neg.sort_unstable();

        let mut i = 0;
        let mut j = 0;

        for s in 0..k as usize {
            let cur_mul = mul as i64 - s as i64;
            let mut best = i64::MIN;
            let mut take = 0;

            if i < pos.len() {
                let x = pos[i];
                let val = x.max(x * cur_mul);

                if val > best {
                    best = val;
                    take = 1;
                }
            }

            if j < neg.len() {
                let x = neg[i];
                let val = x.max(x * cur_mul);

                if val > best {
                    best = val;
                    take = 2;
                }
            }

            if zeros > 0 && 0 > best {
                best = 0;
                take = 3;
            }

            if take == 1 {
                i += 1;
            } else if take == 2 {
                j += 1
            } else {
                zeros -= 1;
            }

            res += best;
        }

        res as i64
    }
}

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        fn solve(nums: &[i32], change: impl Fn(i64) -> i64) -> i64 {
            let first = nums[0] as i64;
            let neg = i64::MIN / 4;

            let mut dp0 = first;
            let mut dp1 = change(first);
            let mut dp2 = neg;

            let mut res = dp1;
            let n = nums.len();

            for i in 1..n {
                let num = nums[i] as i64;
                let x = change(num);
                let nx0 = (dp0 + num).max(num);
                let nx1 = (dp0 + x).max(dp1 + x).max(x);
                let nx2 = (dp2 + num).max(dp1 + num);

                dp0 = nx0;
                dp1 = nx1;
                dp2 = nx2;

                res = res.max(nx1).max(nx2);
            }

            res
        }

        let k = k as i64;
        let multiply = solve(&nums, |x| x * k);
        let divide = solve(&nums, |x| x / k);

        multiply.max(divide)
    }
}

impl Solution {
    pub fn min_time_max_power(
        n: i32,
        edges: Vec<Vec<i32>>,
        power: i32,
        cost: Vec<i32>,
        source: i32,
        target: i32,
    ) -> Vec<i64> {
        let power = power as usize;
        let source = source as usize;
        let target = target as usize;
        let n = n as usize;
        let mut graph = vec![vec![]; n];

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let t = edge[2] as i64;
            graph[u].push((v, t));
        }

        let inf = i64::MAX / 4;
        let mut queue = BinaryHeap::new();
        let mut dist = vec![vec![inf; power + 1]; n];

        queue.push(Reverse((0i64, source, power)));
        dist[source][power] = 0;

        while let Some(Reverse((time, node, remain))) = queue.pop() {
            let need = cost[node] as usize;

            if remain < need {
                continue;
            }

            let next_rem = remain - need;

            for &(next_node, next_time) in &graph[node] {
                if dist[next_node][next_rem] > time + next_time {
                    dist[next_node][next_rem] = time + next_time;
                    queue.push(Reverse((time + next_time, next_node, next_rem)));
                }
            }
        }

        let min_time = dist[target].iter().min().copied().unwrap();

        if min_time == inf {
            return vec![-1, -1];
        }

        let mut max_power = 0i64;

        for remain in 0..=power {
            if dist[target][remain] == min_time {
                max_power = remain as i64;
            }
        }

        vec![min_time, max_power]
    }
}

impl Solution {
    pub fn max_distance(moves: String) -> i32 {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut free = 0;

        for ch in moves.chars() {
            match ch {
                'U' => y += 1,
                'D' => y -= 1,
                'R' => x += 1,
                'L' => x -= 1,
                '_' => free += 1,
                _ => {}
            }
        }

        x.abs() + y.abs() + free
    }
}

impl Solution {
    pub fn count_valid_subarrays(nums: Vec<i32>, x: i32) -> i32 {
        fn first(mut num: i64) -> i64 {
            while num >= 10 {
                num /= 10;
            }
            num
        }

        let n = nums.len() as usize;
        let x = x as i64;
        let mut pref = vec![0i64; n + 1];
        for i in 0..n {
            pref[i + 1] = pref[i] + nums[i] as i64;
        }

        let mut res = 0;

        for i in 0..n {
            for j in i..n {
                let sum = pref[j + 1] - pref[i];

                if sum % 10 == x && first(sum) == x {
                    res += 1;
                }
            }
        }

        res
    }
}

impl Solution {
    pub fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let labels = labels.as_bytes();

        let mut graph = vec![vec![]; n];

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let w = edge[2] as i64;
            graph[u].push((v, w));
        }

        let inf = i64::MAX / 4;
        let mut queue = BinaryHeap::new();
        let mut dist = vec![vec![inf; k + 1]; n];

        dist[0][1] = 0;
        queue.push(Reverse((0, 0usize, 1usize)));

        while let Some(Reverse((cost, node, streak))) = queue.pop() {
            for &(next_node, w) in &graph[node] {
                let next_streak = if labels[node] == labels[next_node] {
                    streak + 1
                } else {
                    1
                };

                if next_streak > k {
                    continue;
                }

                let next_cost = cost + w;

                if next_cost < dist[next_node][next_streak] {
                    dist[next_node][next_streak] = next_cost;
                    queue.push(Reverse((next_cost, next_node, next_streak)));
                }
            }
        }

        let res = dist[n - 1].iter().copied().min().unwrap();

        if res == inf {
            -1
        } else {
            res as i32
        }
    }
}

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

impl Solution {
    pub fn min_operations(s1: String, s2: String) -> i32 {
        fn one_pos_cost(s1: u8, s2: u8, cnt: usize) -> i32 {
            let inf = 1_000_000_000;
            if cnt == 0 {
                return match (s1, s2) {
                    (b'0', b'0') => 0,
                    (b'0', b'1') => 1,
                    (b'1', b'0') => inf,
                    (b'1', b'1') => 0,
                    _ => inf,
                };
            }

            let mut cost = 0;
            if s1 == b'0' {
                cost += 1;
            }

            cost += (cnt - 1) as i32;

            if s2 == b'1' {
                cost += 1
            }

            cost
        }

        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let n = s1.len();
        let inf = 1_000_000_000;
        let mut dp = vec![vec![inf; 2]; n + 1];
        dp[0][0] = 0;

        for i in 0..n {
            for used_prev in 0..=1 {
                if dp[i][used_prev] >= inf {
                    continue;
                }

                for use_cur in 0..=1 {
                    if i + 1 == n && use_cur == 1 {
                        continue;
                    }

                    let cnt = used_prev + use_cur;
                    let cost = one_pos_cost(s1[i], s2[i], cnt);

                    if cost >= inf {
                        continue;
                    }

                    let total = dp[i][used_prev] + cost + use_cur as i32;
                    dp[i + 1][use_cur] = dp[i + 1][use_cur].min(total);
                }
            }
        }

        if dp[n][0] >= inf {
            -1
        } else {
            dp[n][0]
        }
    }
}

impl Solution {
    pub fn get_sum(nums: Vec<i32>) -> i64 {
        const MOD1: i64 = 1_000_000_007;
        const MOD2: i64 = 1_000_000_009;
        const BASE: i64 = 911_382_323;

        fn build_hash(nums: &[i32], modu: i64) -> Vec<i64> {
            let mut h = vec![0i64; nums.len() + 1];
            for i in 0..nums.len() {
                let val = nums[i] as i64 % modu;
                h[i + 1] = (h[i] * BASE + val) % modu;
            }
            h
        }

        fn build_pow(n: usize, modu: i64) -> Vec<i64> {
            let mut p = vec![1i64; n + 1];
            for i in 0..n {
                p[i + 1] = p[i] * BASE % modu;
            }
            p
        }

        fn get_hash(h: &[i64], p: &[i64], l: usize, r: usize, modu: i64) -> i64 {
            let len = r - l + 1;
            (h[r + 1] - h[l] * p[len] % modu + modu) % modu
        }

        let n = nums.len();
        let mut prefix = vec![0i64; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + nums[i] as i64;
        }

        let range_sum = |l: usize, r: usize| -> i64 { prefix[r + 1] - prefix[l] };

        let rev: Vec<i32> = nums.iter().rev().copied().collect();

        let h1 = build_hash(&nums, MOD1);
        let h2 = build_hash(&nums, MOD2);
        let r1 = build_hash(&rev, MOD1);
        let r2 = build_hash(&rev, MOD2);

        let p1 = build_pow(n, MOD1);
        let p2 = build_pow(n, MOD2);

        let is_pal = |l: usize, r: usize| -> bool {
            let rl = n - 1 - r;
            let rr = n - 1 - l;
            get_hash(&h1, &p1, l, r, MOD1) == get_hash(&r1, &p1, rl, rr, MOD1)
                && get_hash(&h2, &p2, l, r, MOD2) == get_hash(&r2, &p2, rl, rr, MOD2)
        };

        let mut res = 0i64;

        for i in 0..n {
            // even
            let mut lo = 1usize;
            let mut hi = (i + 1).min(n - i);
            let mut best = 1usize;

            while lo <= hi {
                let mid = lo + ((hi - lo) >> 1);
                let l = i - mid + 1;
                let r = i + mid - 1;

                if is_pal(l, r) {
                    best = mid;
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            }

            let l = i - best + 1;
            let r = i + best - 1;
            res = res.max(range_sum(l, r));
        }

        for i in 1..n {
            // odd
            let mut lo = 1usize;
            let mut hi = i.min(n - i);
            let mut best = 0usize;

            while lo <= hi {
                let mid = lo + ((hi - lo) >> 1);
                let l = i - mid;
                let r = i + mid - 1;

                if is_pal(l, r) {
                    best = mid;
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            }

            if best > 0 {
                let l = i - best;
                let r = i + best - 1;
                res = res.max(range_sum(l, r));
            }
        }

        res
    }
}
