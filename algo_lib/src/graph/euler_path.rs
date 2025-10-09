use crate::graph::Graph;
use crate::graph::edges::bi_edge::BiEdgeWithId;
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::helpers::ext::option::OptionExt;
use std::collections::HashSet;

pub trait EulerPath {
    fn euler_path(&self) -> Option<Vec<usize>>;
}

impl<P: Clone> EulerPath for Graph<BiEdgeWithId<P>> {
    fn euler_path(&self) -> Option<Vec<usize>> {
        let mut start = 0;
        let mut odd_count = 0;
        for i in 0..self.vertex_count() {
            if self[i].len() % 2 == 1 {
                odd_count += 1;
                start = i
            }
        }
        if odd_count > 2 {
            return None;
        }
        let mut id = vec![0; self.vertex_count()];
        let mut st = vec![start];
        let mut res = Vec::with_capacity(self.edge_count() + 1);
        let mut removed = HashSet::new();

        while let Some(&v) = st.last() {
            while id[v] < self[v].len() && removed.contains(&self[v][id[v]].id()) {
                id[v] += 1;
            }
            if id[v] == self[v].len() {
                st.pop();
                res.push(v);
            } else {
                let edge = &self[v][id[v]];
                removed.insert(edge.id());
                st.push(edge.to());
            }
        }

        let n = res.len();
        res.take_if(n == self.edge_count + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;
    use crate::graph::edges::bi_edge::BiEdgeWithId;
    use std::collections::HashSet;

    fn is_valid_euler_path(g: &Graph<BiEdgeWithId<()>>, path: &[usize]) -> bool {
        if path.len() != g.edge_count() + 1 {
            return false;
        }
        let mut visited_edges = HashSet::new();
        for i in 0..(path.len() - 1) {
            let u = path[i];
            let v = path[i + 1];
            let edge = g[u]
                .iter()
                .find(|e| e.to() == v && !visited_edges.contains(&e.id()));
            if let Some(e) = edge {
                visited_edges.insert(e.id());
            } else {
                return false; // No valid edge found for this step in the path
            }
        }
        visited_edges.len() == g.edge_count()
    }

    #[test]
    fn test_euler_path_exists() {
        // 0 -- 1 -- 2 -- 3 -- 1
        //      |
        //      4
        // Odd degree vertices: 0, 4. Path should exist.
        let mut g = Graph::<BiEdgeWithId<()>>::new(5);
        g.add_edge(BiEdgeWithId::new(0, 1));
        g.add_edge(BiEdgeWithId::new(1, 2));
        g.add_edge(BiEdgeWithId::new(2, 3));
        g.add_edge(BiEdgeWithId::new(3, 1));
        g.add_edge(BiEdgeWithId::new(1, 4));

        let path = g.euler_path();
        assert!(path.is_some());
        assert!(is_valid_euler_path(&g, &path.unwrap()));
    }

    #[test]
    fn test_euler_circuit_exists() {
        // 0 -- 1 -- 2 -- 0
        let mut g = Graph::<BiEdgeWithId<()>>::new(3);
        g.add_edge(BiEdgeWithId::new(0, 1));
        g.add_edge(BiEdgeWithId::new(1, 2));
        g.add_edge(BiEdgeWithId::new(2, 0));

        let path = g.euler_path();
        assert!(path.is_some());
        let p = path.unwrap();
        assert_eq!(p.first(), p.last());
        assert!(is_valid_euler_path(&g, &p));
    }

    #[test]
    fn test_no_euler_path() {
        // 4 odd degree vertices (0, 1, 2, 3)
        let mut g = Graph::<BiEdgeWithId<()>>::new(4);
        g.add_edge(BiEdgeWithId::new(0, 1));
        g.add_edge(BiEdgeWithId::new(2, 3));

        let path = g.euler_path();
        assert!(path.is_none());
    }

    #[test]
    fn test_disconnected_graph() {
        // Two separate cycles: 0-1-2-0 and 3-4-5-3
        let mut g = Graph::<BiEdgeWithId<()>>::new(6);
        g.add_edge(BiEdgeWithId::new(0, 1));
        g.add_edge(BiEdgeWithId::new(1, 2));
        g.add_edge(BiEdgeWithId::new(2, 0));
        g.add_edge(BiEdgeWithId::new(3, 4));
        g.add_edge(BiEdgeWithId::new(4, 5));
        g.add_edge(BiEdgeWithId::new(5, 3));

        let path = g.euler_path();
        assert!(path.is_none());
    }
}
