use crate::collections::arr::arr2d::Arr2d;
use crate::graph::edges::edge_trait::BidirectionalEdgeTrait;
use crate::graph::Graph;
use crate::helpers::owned_cell::OwnedCell;
use crate::numbers::num_traits::bit_ops::BitOps;

pub struct LCA {
    position: Vec<u32>,
    lca_arr: Arr2d<u32>,
    level: Vec<u32>,
    parent: Vec<u32>,
    ancestors: OwnedCell<Option<Arr2d<i32>>>,
}

impl LCA {
    pub fn level(&self, v: usize) -> usize {
        self.level[v] as usize
    }

    pub fn parent(&self, v: usize) -> Option<usize> {
        if (self.parent[v] as usize) == v {
            None
        } else {
            Some(self.parent[v] as usize)
        }
    }

    pub fn lca(&self, first: usize, second: usize) -> usize {
        if first == second {
            first
        } else {
            let from = self.position[first].min(self.position[second]) as usize;
            let to = self.position[first].max(self.position[second]) as usize;
            let lv = (32 - ((to - from) as u32).leading_zeros() - 1) as usize;
            get_min(
                &self.level,
                self.lca_arr[(lv, from)],
                self.lca_arr[(lv, to + 1 - (1 << lv))],
            ) as usize
        }
    }

    pub fn position(&self, vertex: usize) -> usize {
        self.position[vertex] as usize
    }

    pub fn on_path(&self, a: usize, b: usize, c: usize) -> bool {
        let lca = self.lca(a, b);
        self.lca(a, c) == lca && self.lca(b, c) == c || self.lca(a, c) == c && self.lca(b, c) == lca
    }

    pub fn path_length(&self, first: usize, second: usize) -> usize {
        (self.level[first] + self.level[second] - 2 * self.level[self.lca(first, second)]) as usize
    }

    pub fn num_levels(&self) -> usize {
        self.build_steps();
        unsafe { self.ancestors.as_ref().as_ref().unwrap().d1() }
    }

    pub fn nth_ancestor(&self, mut vert: usize, index: usize) -> Option<usize> {
        self.build_steps();
        unsafe {
            let height = self.ancestors.as_ref().as_ref().unwrap().d1();
            if index >= (1 << height) {
                return None;
            }
            for i in 0..height {
                if index.is_set(i) {
                    let pred = self.ancestors.as_ref().as_ref().unwrap()[(i, vert)];
                    if pred == -1 {
                        return None;
                    }
                    vert = pred as usize;
                }
            }
            Some(vert)
        }
    }

    pub fn nth_vert_on_path(&self, from: usize, to: usize, index: usize) -> usize {
        let len = self.path_length(from, to);
        assert!(index <= len);
        let lca = self.lca(from, to);
        if index <= self.level(from) - self.level(lca) {
            self.nth_ancestor(from, index).unwrap()
        } else {
            self.nth_ancestor(to, len - index).unwrap()
        }
    }

    fn build_steps(&self) {
        unsafe {
            if self.ancestors.as_ref().is_some() {
                return;
            }
        }
        let vertex_count = self.position.len();
        let len = (32 - (vertex_count as u32).leading_zeros()) as usize;
        let mut dp = Arr2d::new(len, vertex_count, -1);
        for i in 0..vertex_count {
            dp[(0, i)] = match self.parent(i) {
                None => -1,
                Some(v) => v as i32,
            }
        }

        for i in 1..len {
            for j in 0..vertex_count {
                let x = dp[(i - 1, j)];
                if x == -1 {
                    dp[(i, j)] = -1;
                } else {
                    dp[(i, j)] = dp[(i - 1, x as usize)];
                }
            }
        }

        unsafe {
            self.ancestors.replace(Some(dp));
        }
    }
}

fn get_min(level: &[u32], a: u32, b: u32) -> u32 {
    if level[a as usize] < level[b as usize] {
        a
    } else {
        b
    }
}

pub trait LCATrait {
    fn lca_with_root(&self, root: usize) -> LCA;

    fn lca(&self) -> LCA {
        self.lca_with_root(0)
    }
}

impl<E: BidirectionalEdgeTrait> LCATrait for Graph<E> {
    fn lca_with_root(&self, root: usize) -> LCA {
        debug_assert!(self.is_tree());
        let vertex_count = self.vertex_count();
        let mut order = vec![0u32; 2 * vertex_count - 1];
        let mut pos = vec![vertex_count as u32; vertex_count];
        let mut level = vec![0u32; vertex_count];
        let mut index = vec![0u32; vertex_count];
        let mut parent = vec![0u32; vertex_count];
        let mut stack = vec![0u32; vertex_count];

        stack[0] = root as u32;
        let mut size = 1usize;
        let mut j = 0usize;

        parent[root] = root as u32;
        while size > 0 {
            size -= 1;
            let v = stack[size] as usize;
            if (pos[v] as usize) == vertex_count {
                pos[v] = j as u32;
            }
            order[j] = v as u32;
            j += 1;

            while (index[v] as usize) < self[v].len()
                && (parent[v] as usize) == self[v][index[v] as usize].to()
            {
                index[v] += 1;
            }

            if (index[v] as usize) < self[v].len() {
                stack[size] = v as u32;
                size += 1;
                let to = self[v][index[v] as usize].to();
                stack[size] = to as u32;
                size += 1;
                parent[to] = v as u32;
                level[to] = level[v] + 1;
                index[v] += 1;
            }
        }

        let mut lca_arr = Arr2d::new(
            (32 - ((2 * vertex_count - 1) as u32).leading_zeros()) as usize,
            2 * vertex_count - 1,
            0,
        );

        for i in 0..(2 * vertex_count - 1) {
            lca_arr[(0, i)] = order[i];
        }

        for i in 1..lca_arr.d1() {
            for j in 0..lca_arr.d2() {
                let other = j + (1 << (i - 1));
                if other < lca_arr.d2() {
                    lca_arr[(i, j)] = get_min(&level, lca_arr[(i - 1, j)], lca_arr[(i - 1, other)]);
                } else {
                    lca_arr[(i, j)] = lca_arr[(i - 1, j)];
                }
            }
        }

        LCA {
            position: pos,
            lca_arr,
            level,
            parent,
            ancestors: OwnedCell::new(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LCATrait;
    use crate::graph::edges::bi_edge::BiEdge;
    use crate::graph::Graph;

    // Test tree structure:
    //         0
    //        / \
    //       1   2
    //      / \   \
    //     3   4   5
    //            / \
    //           6   7
    fn create_test_graph() -> Graph<BiEdge<()>> {
        let mut g = Graph::<BiEdge<()>>::new(8);
        g.add_edge(BiEdge::new(0, 1));
        g.add_edge(BiEdge::new(0, 2));
        g.add_edge(BiEdge::new(1, 3));
        g.add_edge(BiEdge::new(1, 4));
        g.add_edge(BiEdge::new(2, 5));
        g.add_edge(BiEdge::new(5, 6));
        g.add_edge(BiEdge::new(5, 7));
        g
    }

    #[test]
    fn test_level_and_parent() {
        let g = create_test_graph();
        let lca = g.lca();

        assert_eq!(lca.level(0), 0);
        assert_eq!(lca.level(1), 1);
        assert_eq!(lca.level(2), 1);
        assert_eq!(lca.level(3), 2);
        assert_eq!(lca.level(4), 2);
        assert_eq!(lca.level(5), 2);
        assert_eq!(lca.level(6), 3);
        assert_eq!(lca.level(7), 3);

        assert_eq!(lca.parent(0), None);
        assert_eq!(lca.parent(1), Some(0));
        assert_eq!(lca.parent(3), Some(1));
        assert_eq!(lca.parent(6), Some(5));
    }

    #[test]
    fn test_lca() {
        let g = create_test_graph();
        let lca = g.lca();

        assert_eq!(lca.lca(3, 4), 1);
        assert_eq!(lca.lca(3, 6), 0);
        assert_eq!(lca.lca(1, 5), 0);
        assert_eq!(lca.lca(6, 7), 5);
        assert_eq!(lca.lca(3, 1), 1);
        assert_eq!(lca.lca(4, 0), 0);
        assert_eq!(lca.lca(7, 7), 7);
    }

    #[test]
    fn test_path_length() {
        let g = create_test_graph();
        let lca = g.lca();

        assert_eq!(lca.path_length(3, 4), 2);
        assert_eq!(lca.path_length(3, 6), 5);
        assert_eq!(lca.path_length(0, 7), 3);
        assert_eq!(lca.path_length(4, 5), 4);
    }

    #[test]
    fn test_nth_ancestor() {
        let g = create_test_graph();
        let lca = g.lca();

        assert_eq!(lca.nth_ancestor(6, 0), Some(6));
        assert_eq!(lca.nth_ancestor(6, 1), Some(5));
        assert_eq!(lca.nth_ancestor(6, 2), Some(2));
        assert_eq!(lca.nth_ancestor(6, 3), Some(0));
        assert_eq!(lca.nth_ancestor(6, 4), None);
        assert_eq!(lca.nth_ancestor(0, 1), None);
    }

    #[test]
    fn test_on_path() {
        let g = create_test_graph();
        let lca = g.lca();

        assert!(lca.on_path(3, 0, 1));
        assert!(lca.on_path(3, 6, 0));
        assert!(lca.on_path(3, 6, 2));
        assert!(!lca.on_path(3, 4, 0));
        assert!(lca.on_path(1, 5, 2));
    }

    #[test]
    fn test_nth_vert_on_path() {
        let g = create_test_graph();
        let lca = g.lca();

        // Path: 3 -> 1 -> 0 -> 2 -> 5 -> 6 (length 5)
        assert_eq!(lca.nth_vert_on_path(3, 6, 0), 3);
        assert_eq!(lca.nth_vert_on_path(3, 6, 1), 1);
        assert_eq!(lca.nth_vert_on_path(3, 6, 2), 0);
        assert_eq!(lca.nth_vert_on_path(3, 6, 3), 2);
        assert_eq!(lca.nth_vert_on_path(3, 6, 4), 5);
        assert_eq!(lca.nth_vert_on_path(3, 6, 5), 6);
    }
}
