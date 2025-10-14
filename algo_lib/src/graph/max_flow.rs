pub trait MaxFlow<C: Ord + Copy> {
    fn max_flow(&mut self, source: usize, destination: usize) -> C;
}
