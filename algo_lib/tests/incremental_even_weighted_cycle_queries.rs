/*
You are given a positive integer n.

There is an undirected graph with n nodes labeled from 0 to n - 1. 
Initially, the graph has no edges.

You are also given a 2D integer array edges, 
where edges[i] = [ui, vi, wi] represents an edge between 
nodes ui and vi with weight wi. 

The weight wi is either 0 or 1.

Process the edges in edges in the given order. 
For each edge, add it to the graph only if, after adding it, 
the sum of the weights of the edges in every cycle in the resulting graph is even.

Return an integer denoting the number of edges 
that are successfully added to the graph.

Example 1:
    Input: n = 3, edges = [[0,1,1],[1,2,1],[0,2,1]]
    Output: 2
    Explanation:
        [0, 1, 1]: We add the edge between vertex 0 and vertex 1 with weight 1.
        [1, 2, 1]: We add the edge between vertex 1 and vertex 2 with weight 1.
        [0, 2, 1]: The edge between vertex 0 and vertex 2 (the dashed edge in the diagram) is not added because the cycle 0 - 1 - 2 - 0 has total edge weight 1 + 1 + 1 = 3, which is an odd number.

Example 2:
    Input: n = 3, edges = [[0,1,1],[1,2,1],[0,2,0]]
    Output: 3
    Explanation:
        [0, 1, 1]: We add the edge between vertex 0 and vertex 1 with weight 1.
        [1, 2, 1]: We add the edge between vertex 1 and vertex 2 with weight 1.
        [0, 2, 0]: We add the edge between vertex 0 and vertex 2 with weight 0.
        Note that the cycle 0 - 1 - 2 - 0 has total edge weight 1 + 1 + 0 = 2, which is an even number.
    
Constraints:
    3 <= n <= 5 * 104
    1 <= edges.length <= 5 * 104
    edges[i] = [ui, vi, wi]
    0 <= ui < vi < n
    All edges are distinct.
    wi = 0 or wi = 1
*/



struct UnionFind {
    parent: Vec<usize>,
    weight: Vec<i32>,
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

        // find cycle
        if root_x == root_y {
            // (self.weight[x] + self.weight[y]) % 2 == val;
            return (self.weight[x] ^ self.weight[y]) == val;
        }

        if self.size[root_x] > self.size[root_y] {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
            // self.weight[root_y] = (self.weight[x] + self.weight[y] + val) % 2;
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
            let v = edge[1] as usize;
            let w = edge[2];

            if uf.union(u, v, w) {
                res += 1;
            }
        }

        res
    }
}