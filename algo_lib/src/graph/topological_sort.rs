use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::Graph;
use std::collections::VecDeque;

pub trait TopoSort {
    fn topo_sort(&self) -> Option<Vec<usize>>;
}

impl<E: EdgeTrait> TopoSort for Graph<E> {
    fn topo_sort(&self) -> Option<Vec<usize>> {
        assert!(!E::REVERSABLE);
        let n = self.vertex_count();
        let mut res = Vec::with_capacity(n);
        let mut degree = vec![0u32; n];
        for v in 0..n {
            for u in self[v].iter() {
                degree[u.to()] += 1;
            }
        }

        let mut queue = VecDeque::new();
        for (v, deg) in degree.iter().enumerate() {
            if *deg == 0 {
                queue.push_back(v);
            }
        }

        while !queue.is_empty() {
            let v = queue.pop_front().unwrap();
            res.push(v);
            for e in self[v].iter() {
                let u = e.to();
                degree[u] -= 1;
                if degree[u] == 0 {
                    queue.push_back(u)
                }
            }
        }

        if res.len() == n {
            Some(res)
        } else {
            None
        }
    }
}
