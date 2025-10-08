use crate::collections::dsu::DSU;
use crate::graph::Graph;
use crate::graph::edges::edge_weighted::WeightedEdgeTrait;

pub trait MinimalSpanningTree<W: Ord + Copy, E: WeightedEdgeTrait<W>> {
    fn minimal_spanning_tree(&self) -> Graph<E>;
}

impl<W: Ord + Copy, E: WeightedEdgeTrait<W> + Clone> MinimalSpanningTree<W, E> for Graph<E> {
    fn minimal_spanning_tree(&self) -> Graph<E> {
        let mut edges = Vec::with_capacity(self.edge_count());
        for v in 0..self.vertex_count() {
            for e in &self[v] {
                if e.to() > v {
                    edges.push((v, e.clone()));
                }
            }
        }
        edges.sort_by_key(|(_, e)| e.weight());
        let mut res = Graph::new(self.vertex_count());
        let mut dsu = DSU::new(self.vertex_count());

        for (v, e) in edges {
            if dsu.union(v, e.to()) {
                res.add_edge((v, e));
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::edges::edge_weighted::{WeightedEdgeTrait, WeightedEdgeWithId};

    fn get_total_weight<
        W: Ord + Copy + Default + std::ops::AddAssign,
        E: WeightedEdgeTrait<W> + Clone,
    >(
        graph: &Graph<E>,
    ) -> W {
        let mut res = Default::default();

        for v in 0..graph.vertex_count() {
            for e in &graph[v] {
                if v < e.to() {
                    res += e.weight();
                }
            }
        }

        res
    }

    #[test]
    fn test_minimal_spanning_tree() {
        let mut graph = Graph::<WeightedEdgeWithId<i32, ()>>::new(7);
        graph.add_edge((0, WeightedEdgeWithId::new(1, 7)));
        graph.add_edge((0, WeightedEdgeWithId::new(3, 5)));
        graph.add_edge((1, WeightedEdgeWithId::new(2, 8)));
        graph.add_edge((1, WeightedEdgeWithId::new(3, 9)));
        graph.add_edge((1, WeightedEdgeWithId::new(4, 7)));
        graph.add_edge((2, WeightedEdgeWithId::new(4, 5)));
        graph.add_edge((3, WeightedEdgeWithId::new(4, 15)));
        graph.add_edge((3, WeightedEdgeWithId::new(5, 6)));
        graph.add_edge((4, WeightedEdgeWithId::new(5, 8)));
        graph.add_edge((4, WeightedEdgeWithId::new(6, 9)));
        graph.add_edge((5, WeightedEdgeWithId::new(6, 11)));

        let mst = graph.minimal_spanning_tree();

        assert_eq!(get_total_weight(&mst), 39);
        assert_eq!(mst.edge_count(), 6);
    }

    #[test]
    fn test_mst_empty_graph() {
        let graph = Graph::<WeightedEdgeWithId<i32, ()>>::new(0);
        let mst = graph.minimal_spanning_tree();
        assert_eq!(mst.vertex_count(), 0);
        assert_eq!(mst.edge_count(), 0);
    }

    #[test]
    fn test_mst_no_edges() {
        let graph = Graph::<WeightedEdgeWithId<i32, ()>>::new(5);
        let mst = graph.minimal_spanning_tree();
        assert_eq!(mst.vertex_count(), 5);
        assert_eq!(mst.edge_count(), 0);
        assert_eq!(get_total_weight(&mst), 0);
    }

    #[test]
    fn test_mst_disconnected_graph() {
        let mut graph = Graph::<WeightedEdgeWithId<i32, ()>>::new(5);
        // Component 1
        graph.add_edge((0, WeightedEdgeWithId::new(1, 10)));
        graph.add_edge((0, WeightedEdgeWithId::new(2, 20)));
        // Component 2
        graph.add_edge((3, WeightedEdgeWithId::new(4, 5)));

        let mst = graph.minimal_spanning_tree();
        assert_eq!(mst.vertex_count(), 5);
        assert_eq!(mst.edge_count(), 3); // V - C = 5 - 2 = 3
        assert_eq!(get_total_weight(&mst), 10 + 20 + 5);
    }
}
