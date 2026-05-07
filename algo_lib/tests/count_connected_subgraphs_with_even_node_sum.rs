/*
You are given an undirected graph with n nodes labeled from 0 to n - 1. 
Node i has a value of nums[i], which is either 0 or 1. 
The edges of the graph are given by a 2D array edges 
where edges[i] = [ui, vi] represents an edge between node ui and node vi.

For a non-empty subset s of nodes in the graph, we consider the 
induced subgraph of s generated as follows:

We keep only the nodes in s.
We keep only the edges whose two endpoints are both in s.
Return an integer representing the number of non-empty 
subsets s of nodes in the graph such that:

The induced subgraph of s is connected.
The sum of node values in s is even.
 
Example 1:
    Input: nums = [1,0,1], edges = [[0,1],[1,2]]
    Output: 2
    Explanation:
        s	connected?	sum of node values	counted?
        [0]	Yes	1	No
        [1]	Yes	0	Yes
        [2]	Yes	1	No
        [0,1]	Yes	1	No
        [0,2]	No, node 0 and node 2 are disconnected.	2	No
        [1,2]	Yes	1	No
        [0,1,2]	Yes	2	Yes

Example 2:
    Input: nums = [1], edges = []
    Output: 0
    Explanation:
        s	connected?	sum of node values	counted?
        [0]	Yes	1	No
    

Constraints:
    1 <= n == nums.length <= 13
    nums[i] is 0 or 1.
    0 <= edges.length <= n * (n - 1) / 2
    edges[i] = [ui, vi]
    0 <= ui < vi < n
    All edges are distinct.

*/


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


impl Solution {
    pub fn even_sum_subgraphs(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut res = 0;

        for mask in 1..(1<<n) {
            let mut sum = 0;
            let mut nodes_in_subset = 0;

            for i in 0..n {
                if (mask & (1 << i)) != 0 {
                    sum += nums[i];
                    nodes_in_subset += 1;
                }
            }

            if sum % 2 != 0 {
                continue;
            }

            let mut uf = UF::new(n);
            let mut components = nodes_in_subset;

            for edge in &edges {
                let u = edge[0] as usize;
                let v = edge[1] as usize;

                if (mask & (1 << u)) != 0 && (mask & (1 << v)) != 0 {
                    if uf.union(u, v) {
                        components -= 1;
                    }
                }
            }

            if components == 1 {
                res += 1
            }
        }

        res
    }
}
