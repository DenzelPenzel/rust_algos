use crate::graph::Graph;
use crate::graph::edges::edge::Edge;
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::helpers::recursive_function::{Callable, RecursiveFunction};
use std::collections::HashSet;

pub struct SCC {
    pub color: Vec<usize>,
    pub edges: Graph<Edge<()>>,
}

pub trait SCCTrait {
    fn scc(&self) -> SCC;
}

impl<E: EdgeTrait> SCCTrait for Graph<E> {
    fn scc(&self) -> SCC {
        assert!(!E::REVERSABLE);
        let n = self.vertex_count();
        let mut order = Vec::with_capacity(n);
        let mut color = vec![0; n];
        let mut visited = HashSet::new();
        let mut res = Graph::new(0);

        for i in 0..n {
            if !visited.contains(&i) {
                let mut first_dfs = RecursiveFunction::new(|f, vert| {
                    if visited.contains(&vert) {
                        return;
                    }
                    visited.insert(vert);
                    for e in self[vert].iter() {
                        f.call(e.to());
                    }
                    order.push(vert);
                });
                first_dfs.call(i);
            }
        }

        visited = HashSet::new();
        let mut index = 0usize;
        let mut graph = Graph::new(n);
        let mut next = vec![n; n];
        let mut queue = Vec::with_capacity(n);

        for v in 0..n {
            for u in self[v].iter() {
                graph.add_edge(Edge::new(u.to(), v));
            }
        }

        for i in (0..n).rev() {
            if !visited.contains(&order[i]) {
                let key = i;

                let mut second_dfs = RecursiveFunction::new(|f, vert| {
                    if visited.contains(&vert) {
                        if color[vert] != index && next[color[vert]] != key {
                            next[color[vert]] = key;
                            queue.push(color[vert]);
                        }
                        return;
                    }
                    color[vert] = index;
                    visited.insert(vert);
                    for e in graph[vert].iter() {
                        f.call(e.to());
                    }
                });

                second_dfs.call(order[i]);
                res.add_vertices(1);

                for j in queue.drain(..) {
                    res.add_edge(Edge::new(j, index));
                }

                index += 1;
            }
        }

        SCC { color, edges: res }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scc() {
        let mut graph = Graph::new(8);
        graph.add_edge(Edge::new(0, 1));
        graph.add_edge(Edge::new(1, 2));
        graph.add_edge(Edge::new(2, 0));

        graph.add_edge(Edge::new(2, 3));

        graph.add_edge(Edge::new(3, 4));
        graph.add_edge(Edge::new(4, 5));
        graph.add_edge(Edge::new(5, 3));

        graph.add_edge(Edge::new(5, 6));
        graph.add_edge(Edge::new(6, 7));

        let scc = graph.scc();
        let colors = scc.color;

        let mut components: Vec<Vec<usize>> = Vec::new();
        for (i, &c) in colors.iter().enumerate() {
            if c >= components.len() {
                components.resize(c + 1, Vec::new());
            }
            components[c].push(i);
        }

        for component in &mut components {
            component.sort();
        }
        components.retain(|c| !c.is_empty());
        components.sort_by_key(|c| c[0]);

        assert_eq!(components.len(), 4);
        assert_eq!(components[0], vec![0, 1, 2]);
        assert_eq!(components[1], vec![3, 4, 5]);
        assert_eq!(components[2], vec![6]);
        assert_eq!(components[3], vec![7]);
    }
}
