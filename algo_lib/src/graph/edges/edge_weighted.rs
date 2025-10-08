use crate::graph::edges::edge_id::{EdgeId, NoId, WithId};
use crate::graph::edges::edge_trait::EdgeTrait;

pub trait WeightedEdgeTrait<W: Copy>: EdgeTrait {
    fn weight(&self) -> W;
    fn weight_mut(&mut self) -> &mut W;
}

#[derive(Clone)]
pub struct WeightedEdgeRaw<W: Copy, Id: EdgeId, P> {
    to: u32,
    weight: W,
    id: Id,
    payload: P,
}

impl<W: Copy, Id: EdgeId> WeightedEdgeRaw<W, Id, ()> {
    pub fn new(to: usize, w: W) -> Self {
        Self {
            to: to as u32,
            weight: w,
            id: Id::new(),
            payload: (),
        }
    }
}

impl<W: Copy, Id: EdgeId, P> WeightedEdgeRaw<W, Id, P> {
    pub fn with_payload(from: usize, to: usize, w: W, payload: P) -> (usize, Self) {
        (from, Self::with_payload_impl(to, w, payload))
    }

    fn with_payload_impl(to: usize, w: W, payload: P) -> Self {
        Self {
            to: to as u32,
            weight: w,
            id: Id::new(),
            payload,
        }
    }
}

impl<W: Copy, Id: EdgeId, P: Clone> EdgeTrait for WeightedEdgeRaw<W, Id, P> {
    type Payload = P;
    const REVERSABLE: bool = false;

    fn to(&self) -> usize {
        self.to as usize
    }

    fn id(&self) -> usize {
        self.id.id()
    }

    fn set_id(&mut self, id: usize) {
        self.id.set_id(id);
    }

    fn reverse_id(&self) -> usize {
        panic!("no reverse")
    }

    fn set_reverse_id(&mut self, _: usize) {
        panic!("no reverse")
    }

    fn reverse_edge(&self, _: usize) -> Self {
        panic!("no reverse")
    }

    fn payload(&self) -> &P {
        &self.payload
    }
}

impl<W: Copy, Id: EdgeId, P: Clone> WeightedEdgeTrait<W> for WeightedEdgeRaw<W, Id, P> {
    fn weight(&self) -> W {
        self.weight
    }

    fn weight_mut(&mut self) -> &mut W {
        &mut self.weight
    }
}

pub type WeightedEdge<W, P> = WeightedEdgeRaw<W, NoId, P>;
pub type WeightedEdgeWithId<W, P> = WeightedEdgeRaw<W, WithId, P>;
