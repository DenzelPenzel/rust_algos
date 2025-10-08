use std::ops::Range;
use crate::graph::edges::edge_trait::BidirectionalEdgeTrait;
use crate::graph::Graph;

pub trait DFSOrderTrait {
    fn dfs_order_with_root(&self, root: usize) -> DFSOrder;

    fn dfs_order(&self) -> DFSOrder {
        self.dfs_order_with_root(0)
    }
}

pub struct DFSOrder {
    pub position: Vec<usize>,
    pub end: Vec<usize>,
}

impl DFSOrder {
    // get the size of the subtree rooted at v
    pub fn len(&self, v: usize) -> usize {
        self.end[v] - self.position[v]
    }

    pub fn subtree(&self, v: usize) -> Range<usize> {
        self.position[v]..self.end[v]
    }
}


impl<E: BidirectionalEdgeTrait> DFSOrderTrait for Graph<E> {
    fn dfs_order_with_root(&self, root: usize) -> DFSOrder {
        debug_assert!(self.is_tree());
        let count = self.vertex_count();
        // store entry time
        let mut position = vec![0; count];
        // store exit time
        let mut end = vec![0; count];
        let mut edge = vec![0u32; count];
        let mut stack = vec![0u32; count];
        let mut last = vec![0u32; count];
        let mut size = 1usize;
        last[root] = root as u32;
        stack[0] = root as u32;
        position[root] = 0;
        let mut index = 0usize;

        while size > 0 {
            let current = stack[size - 1] as usize;
            let c_edge = &mut edge[current];
            if *c_edge == self[current].len() as u32 {
                end[current] = index + 1;
                size -= 1;
            } else {
                let next = self[current][*c_edge as usize].to();
                *c_edge += 1;
                if next == (last[current] as usize) {
                    continue;
                }
                index += 1;
                position[next] = index;
                last[next] = current as u32;
                stack[size] = next as u32;
                size += 1;
            }
        }

        DFSOrder {
            position,
            end
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::edges::bi_edge::BiEdge;

    #[test]
    fn test_dfs_order() {
        // A tree that looks like:
        // 0 -- 1 -- 3
        // | \
        // |  4
        // 2
        let mut graph = Graph::<BiEdge<()>>::new(5);
        graph.add_edge(BiEdge::new(0, 1));
        graph.add_edge(BiEdge::new(0, 2));
        graph.add_edge(BiEdge::new(0, 4));
        graph.add_edge(BiEdge::new(1, 3));

        let dfs_order = graph.dfs_order_with_root(0);

        // The order of visiting children is dependent on the graph implementation.
        // Assuming adjacency lists store neighbors in the order they are added:
        // Neighbors of 0: [1, 2, 4]
        // Neighbors of 1: [0, 3]
        // Expected pre-order traversal: 0, 1, 3, 2, 4
        let expected_traversal = vec![0, 1, 3, 2, 4];
        let mut actual_traversal = vec![0; 5];
        for i in 0..5 {
            actual_traversal[dfs_order.position[i]] = i;
        }
        assert_eq!(actual_traversal, expected_traversal);

        // Check subtree sizes
        assert_eq!(dfs_order.len(0), 5);
        assert_eq!(dfs_order.len(1), 2);
        assert_eq!(dfs_order.len(2), 1);
        assert_eq!(dfs_order.len(3), 1);
        assert_eq!(dfs_order.len(4), 1);

        // Check subtree ranges
        assert_eq!(dfs_order.subtree(0), 0..5);
        assert_eq!(dfs_order.subtree(1), 1..3);
        assert_eq!(dfs_order.subtree(2), 3..4);
        assert_eq!(dfs_order.subtree(3), 2..3);
        assert_eq!(dfs_order.subtree(4), 4..5);
    }
}
