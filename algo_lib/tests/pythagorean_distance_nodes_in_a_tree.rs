/*
You are given an integer n and an undirected tree with
n nodes numbered from 0 to n - 1. The tree is represented by a 2D array edges of length n - 1,
where edges[i] = [ui, vi] indicates an undirected edge between ui and vi.

You are also given three distinct target nodes x, y, and z.

For any node u in the tree:

Let dx be the distance from u to node x
Let dy be the distance from u to node y
Let dz be the distance from u to node z
The node u is called special if the three distances form a Pythagorean Triplet.

Return an integer denoting the number of special nodes in the tree.

A Pythagorean triplet consists of three integers a, b, and c which,
when sorted in ascending order, satisfy a2 + b2 = c2.

The distance between two nodes in a tree is the number of edges on the unique path between them.

Example 1:
    Input: n = 4, edges = [[0,1],[0,2],[0,3]], x = 1, y = 2, z = 3
    Output: 3
    Explanation:
        For each node, we compute its distances to nodes x = 1, y = 2, and z = 3.

        Node 0 has distances 1, 1, and 1. After sorting, the distances are 1, 1, and 1, which do not satisfy the Pythagorean condition.
        Node 1 has distances 0, 2, and 2. After sorting, the distances are 0, 2, and 2. Since 02 + 22 = 22, node 1 is special.
        Node 2 has distances 2, 0, and 2. After sorting, the distances are 0, 2, and 2. Since 02 + 22 = 22, node 2 is special.
        Node 3 has distances 2, 2, and 0. After sorting, the distances are 0, 2, and 2. This also satisfies the Pythagorean condition.
        Therefore, nodes 1, 2, and 3 are special, and the answer is 3.

Example 2:
    Input: n = 4, edges = [[0,1],[1,2],[2,3]], x = 0, y = 3, z = 2
    Output: 0
    Explanation:
        For each node, we compute its distances to nodes x = 0, y = 3, and z = 2.

        Node 0 has distances 0, 3, and 2. After sorting, the distances are 0, 2, and 3, which do not satisfy the Pythagorean condition.
        Node 1 has distances 1, 2, and 1. After sorting, the distances are 1, 1, and 2, which do not satisfy the Pythagorean condition.
        Node 2 has distances 2, 1, and 0. After sorting, the distances are 0, 1, and 2, which do not satisfy the Pythagorean condition.
        Node 3 has distances 3, 0, and 1. After sorting, the distances are 0, 1, and 3, which do not satisfy the Pythagorean condition.
        No node satisfies the Pythagorean condition. Therefore, the answer is 0.

Example 3:
    Input: n = 4, edges = [[0,1],[1,2],[1,3]], x = 1, y = 3, z = 0
    Output: 1
    Explanation:
        For each node, we compute its distances to nodes x = 1, y = 3, and z = 0.

        Node 0 has distances 1, 2, and 0. After sorting, the distances are 0, 1, and 2, which do not satisfy the Pythagorean condition.
        Node 1 has distances 0, 1, and 1. After sorting, the distances are 0, 1, and 1. Since 02 + 12 = 12, node 1 is special.
        Node 2 has distances 1, 2, and 2. After sorting, the distances are 1, 2, and 2, which do not satisfy the Pythagorean condition.
        Node 3 has distances 1, 0, and 2. After sorting, the distances are 0, 1, and 2, which do not satisfy the Pythagorean condition.
        Therefore, the answer is 1.

Constraints:
    4 <= n <= 10^5
    edges.length == n - 1
    edges[i] = [ui, vi]
    0 <= ui, vi, x, y, z <= n - 1
    x, y, and z are pairwise distinct.
    The input is generated such that edges represent a valid tree.
*/

use std::mem;

struct Solution;

struct TreeDistance {
    n: usize,
    log: usize,
    depth: Vec<usize>,
    // up[u][i] stores the 2^i-th ancestor of u
    up: Vec<Vec<usize>>,
}

impl TreeDistance {
    // LCA (longest common ancestor)
    pub fn new(n: usize, edges: &[(usize, usize)], root: usize) -> Self {
        let mut adj = vec![vec![]; n as usize];

        for &(u, v) in edges {
            adj[u].push(v);
            adj[v].push(u);
        }

        // Calculate max power of 2 needed (log2(10^5) is approx 17)
        let log = (usize::BITS - n.leading_zeros()) as usize;
        let mut depth = vec![0; n as usize];
        let mut up = vec![vec![root; log]; n as usize];

        let mut stack = vec![(root, root, 0)];

        while let Some((u, p, d)) = stack.pop() {
            depth[u] = d;
            up[u][0] = p; // 2^0 ancestor is parent

            for &v in &adj[u] {
                if v == p {
                    continue;
                }
                stack.push((v, u, d + 1));
            }
        }

        for j in 1..log {
            for i in 0..n {
                let parent_mid = up[i][j - 1];
                up[i][j] = up[parent_mid][j - 1];
            }
        }

        TreeDistance { n, log, depth, up }
    }

    pub fn find_lca(&self, mut u: usize, mut v: usize) -> usize {
        // Ensure u is the deeper node
        if self.depth[u] < self.depth[v] {
            mem::swap(&mut u, &mut v);
        }

        // Lift u up to the same depth as v
        let diff = self.depth[u] - self.depth[v];
        for i in 0..self.log {
            if (diff >> i) & 1 == 1 {
                u = self.up[u][i];
            }
        }

        if u == v {
            return u;
        }

        // Lift both u and v until they are just below the LCA
        for i in (0..self.log).rev() {
            if self.up[u][i] != self.up[v][i] {
                u = self.up[u][i];
                v = self.up[v][i];
            }
        }

        // The parent of u (or v) is the LCA
        self.up[u][0]
    }

    pub fn get_dist(&self, u: usize, v: usize) -> usize {
        let lca = self.find_lca(u, v);
        self.depth[u] + self.depth[v] - 2 * self.depth[lca]
    }
}

impl Solution {
    pub fn special_nodes(n: i32, edges: Vec<Vec<i32>>, x: i32, y: i32, z: i32) -> i32 {
        let mut res = 0;
        let n = n as usize;
        let edges: Vec<(usize, usize)> = edges
            .iter()
            .map(|p| (p[0] as usize, p[1] as usize))
            .collect();

        let tree = TreeDistance::new(n, &edges, 0);

        for u in 0..n {
            let dx = tree.get_dist(u, x as usize);
            let dy = tree.get_dist(u, y as usize);
            let dz = tree.get_dist(u, z as usize);

            let mut dists = vec![dx, dy, dz];
            dists.sort_unstable();

            if dists[0].pow(2) + dists[1].pow(2) == dists[2].pow(2) {
                res += 1;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let n = 4;
        let edges = vec![vec![0, 1], vec![0, 2], vec![0, 3]];
        let x = 1;
        let y = 2;
        let z = 3;
        assert_eq!(Solution::special_nodes(n, edges, x, y, z), 3);
    }

    #[test]
    fn test_example_2() {
        let n = 4;
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let x = 0;
        let y = 3;
        let z = 2;
        assert_eq!(Solution::special_nodes(n, edges, x, y, z), 0);
    }

    #[test]
    fn test_example_3() {
        let n = 4;
        let edges = vec![vec![0, 1], vec![1, 2], vec![1, 3]];
        let x = 1;
        let y = 3;
        let z = 0;
        assert_eq!(Solution::special_nodes(n, edges, x, y, z), 1);
    }
}
