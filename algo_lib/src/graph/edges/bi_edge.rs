use crate::graph::edges::edge_id::{EdgeId, NoId, WithId};
use crate::graph::edges::edge_trait::{BidirectionalEdgeTrait, EdgeTrait};

#[derive(Clone)]
pub struct BiEdgeRaw<Id: EdgeId, P> {
    to: u32,
    id: Id,
    reverse_id: usize,
    payload: P,
}

impl<Id: EdgeId> BiEdgeRaw<Id, ()> {
    pub fn new(from: usize, to: usize) -> (usize, Self) {
        (
            from,
            Self {
                to: to as u32,
                id: Id::new(),
                reverse_id: 0,
                payload: (),
            },
        )
    }
}

impl<Id: EdgeId, P: Clone> EdgeTrait for BiEdgeRaw<Id, P> {
    type Payload = P;

    const REVERSABLE: bool = true;

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
        self.reverse_id
    }

    fn set_reverse_id(&mut self, reverse_id: usize) {
        self.reverse_id = reverse_id;
    }

    fn reverse_edge(&self, from: usize) -> Self {
        Self {
            to: from as u32,
            id: self.id.clone(),
            reverse_id: 0, // This will be set by the graph
            payload: self.payload.clone(),
        }
    }

    fn payload(&self) -> &P {
        &self.payload
    }
}

impl<Id: EdgeId, P: Clone> BidirectionalEdgeTrait for BiEdgeRaw<Id, P> {}

pub type BiEdge<P> = BiEdgeRaw<NoId, P>;
pub type BiEdgeWithId<P> = BiEdgeRaw<WithId, P>;
