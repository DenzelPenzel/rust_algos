/*
You are given a directed, weighted graph with n nodes labeled from 0 to n - 1, 
and an array edges where edges[i] = [ui, vi, wi] represents a directed edge from node ui to node vi with cost wi.

Each node ui has a switch that can be used at most once: when you arrive at ui and have not yet used its switch, 
you may activate it on one of its incoming edges vi → ui reverse that edge to ui → vi and immediately traverse it.

The reversal is only valid for that single move, and using a reversed edge costs 2 * wi.

Return the minimum total cost to travel from node 0 to node n - 1. If it is not possible, return -1.

Example 1:
    Input: n = 4, edges = [[0,1,3],[3,1,1],[2,3,4],[0,2,2]]
    Output: 5
    Explanation:
        Use the path 0 → 1 (cost 3).
        At node 1 reverse the original edge 3 → 1 into 1 → 3 and traverse it at cost 2 * 1 = 2.
        Total cost is 3 + 2 = 5.

Example 2:
    Input: n = 4, edges = [[0,2,1],[2,1,1],[1,3,1],[2,3,3]]
    Output: 3
    Explanation:
        No reversal is needed. Take the path 0 → 2 (cost 1), then 2 → 1 (cost 1), then 1 → 3 (cost 1).
        Total cost is 1 + 1 + 1 = 3.
    
Constraints:
    2 <= n <= 5 * 104
    1 <= edges.length <= 105
    edges[i] = [ui, vi, wi]
    0 <= ui, vi <= n - 1
    1 <= wi <= 1000

*/


use std::{collections::VecDeque, i32};

struct Solution;

impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![]; n];        

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let w = edge[2];
            graph[u].push((v, w));
            graph[v].push((u, w * 2));
        }

        let mut queue = VecDeque::new();
        queue.push_back((0, 0)); // (node, used_reverse_flag)

        let mut dist = vec![i32::MAX; n];

        // Performance filter, ensures that node V exists in the queue at most once
        // If V is already waiting to be processed, and another neighbor 
        // finds an even better path to V, we just update the distance value for V where it sits
        // We don't need to add it to the queue again because when the queue finally gets to V 
        // it will use that latest, best value anyway
        let mut in_queue = vec![false; n];

        in_queue[0] = true;
        dist[0] = 0;
    

        while let Some((u, used)) = queue.pop_front() {
            // Important reset; without this time O(2^N) instead of O(E)
            in_queue[u] = false;

            for &(v, w) in &graph[u] {
                if dist[u] != i32::MAX && dist[u] + w < dist[v] {
                    dist[v] = dist[u] + w;
                    if !in_queue[v] {
                        queue.push_back((v, used));
                        in_queue[v] = true;
                    }
                }
            }
        }

        if dist[n - 1] == i32::MAX { -1 } else { dist[n - 1] } 
    }
}