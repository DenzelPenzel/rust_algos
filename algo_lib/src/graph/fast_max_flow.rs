use crate::collections::link_cut::LinkCutNode;
use crate::collections::payload::Payload;
use crate::collections::vec_ext::gen_vec::VecGen;
use crate::graph::edges::flow_edge_trait::FlowEdgeTrait;
use crate::graph::Graph;
use crate::numbers::num_traits::algebra::AdditionMonoidWithSub;
use crate::numbers::ord::MinMax;

pub trait FastMaxFlow<C: AdditionMonoidWithSub + Ord + Copy + MinMax> {
    fn fast_max_flow(&mut self, source: usize, target: usize) -> C;
}

impl<C: AdditionMonoidWithSub + Ord + Copy + MinMax, E: FlowEdgeTrait<C>> FastMaxFlow<C>
    for Graph<E>
{
    fn fast_max_flow(&mut self, source: usize, target: usize) -> C {
        struct Node<C: AdditionMonoidWithSub + Ord + Copy + MinMax> {
            id: u32,
            self_val: C,
            val: C,
            val_id: u32,
            delta: C,
            pushed: C,
        }

        impl<C: AdditionMonoidWithSub + Ord + Copy + MinMax> Payload for Node<C> {
            const NEED_UPDATE: bool = true;
            const NEED_ACCUMULATE: bool = true;
            fn reset_delta(&mut self) {
                self.delta = C::zero();
            }

            fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
                self.val_id = self.id;
                self.val = self.self_val;
                if let Some(l) = left {
                    if l.val < self.val {
                        self.val = l.val;
                        self.val_id = l.val_id;
                    }
                }
                if let Some(r) = right {
                    if r.val < self.val {
                        self.val = r.val;
                        self.val_id = r.val_id;
                    }
                }
            }

            fn accumulate(&mut self, delta: &Self) {
                self.self_val += delta.delta;
                self.val += delta.delta;
                self.delta += delta.delta;
                self.pushed -= delta.delta;
            }
        }

        let n = self.vertex_count();

        // let nodes = Vec::with_gen(n, |i| {
        //
        // })

        let total_flow = C::zero();

        total_flow
    }
}
