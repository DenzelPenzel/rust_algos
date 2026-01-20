use crate::collections::min_max::MinimMaxim;
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::Graph;
use crate::helpers::recursive_function::{Callable2, RecursiveFunction2};
use std::collections::HashSet;

pub trait BridgeSearch {
    fn bridges(&self) -> Vec<(usize, usize)>;
}

impl<E: EdgeTrait> BridgeSearch for Graph<E> {
    fn bridges(&self) -> Vec<(usize, usize)> {
        assert!(E::REVERSABLE);
        let n = self.vertex_count();
        let mut time = 0;
        let mut tin = vec![0; n];
        let mut fup = vec![0; n];
        let mut used = HashSet::<usize>::new();
        let mut res = Vec::new();

        for i in 0..n {
            if !used.contains(&i) {
                let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
                    used.insert(vert);
                    tin[vert] = time;
                    fup[vert] = time;
                    time += 1;

                    let mut first = true;

                    for u in self[vert].iter() {
                        if u.to() == prev && first {
                            first = false;
                            continue;
                        }
                        let to = u.to();
                        if used.contains(&to) {
                            fup[vert].minim(tin[to]);
                        } else {
                            f.call(to, vert);
                            let cand = fup[to];
                            fup[vert].minim(cand);
                            if fup[to] > tin[vert] {
                                res.push((vert, to))
                            }
                        }
                    }
                });
                dfs.call(i, i);
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::BridgeSearch;
    use crate::graph::edges::bi_edge::BiEdge;
    use crate::graph::Graph;

    //   0 -- 2
    //   |  /
    //   | /
    //   1 --- 3 --- 4
    //
    // Bridges are (1, 3) and (3, 4)
    #[test]
    fn test_bridges() {
        let mut g = Graph::<BiEdge<()>>::new(5);
        g.add_edge(BiEdge::new(0, 1));
        g.add_edge(BiEdge::new(0, 2));
        g.add_edge(BiEdge::new(1, 2));
        g.add_edge(BiEdge::new(1, 3));
        g.add_edge(BiEdge::new(3, 4));

        let mut bridges = g.bridges();
        bridges.sort();

        let mut expected = vec![(1, 3), (3, 4)];
        expected.sort();

        assert_eq!(bridges, expected);
    }

    // 0 --- 1
    // |     |
    // 3 --- 2
    //
    // No bridges in a cycle.
    #[test]
    fn test_no_bridges() {
        let mut g = Graph::<BiEdge<()>>::new(4);
        g.add_edge(BiEdge::new(0, 1));
        g.add_edge(BiEdge::new(1, 2));
        g.add_edge(BiEdge::new(2, 3));
        g.add_edge(BiEdge::new(3, 0));

        let bridges = g.bridges();
        assert!(bridges.is_empty());
    }
}
