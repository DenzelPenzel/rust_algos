use crate::collections::bounds::clamp;
use crate::collections::min_max::MinimMaxim;
use crate::numbers::num_traits::algebra::AdditionMonoidWithSub;
use std::ops::RangeBounds;

#[derive(Clone)]
pub struct FenwickTree<T> {
    value: Vec<T>,
}

impl<T: AdditionMonoidWithSub + Copy> FenwickTree<T> {
    pub fn new(size: usize) -> Self {
        Self {
            value: vec![T::zero(); size],
        }
    }

    pub fn get_to(&self, mut to: usize) -> T {
        to.minim(self.value.len());
        let mut res = T::zero();
        while to > 0 {
            to -= 1;
            res += self.value[to];
            to &= to + 1;
        }
        res
    }

    pub fn get(&self, bounds: impl RangeBounds<usize>) -> T {
        let (from, to) = clamp(&bounds, self.value.len());
        if from >= to {
            T::zero()
        } else {
            self.get_to(to) - self.get_to(from)
        }
    }

    pub fn add(&mut self, mut idx: usize, v: T) {
        while idx < self.value.len() {
            self.value[idx] += v;
            idx |= idx + 1;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        self.value.iter().enumerate().map(|(i, _)| self.get(i..=i))
    }

    pub fn clear(&mut self) {
        self.value.fill(T::zero());
    }
}

impl<T: AdditionMonoidWithSub + Copy> From<&[T]> for FenwickTree<T> {
    fn from(slice: &[T]) -> Self {
        let mut res = Self::new(slice.len());
        for (idx, &v) in slice.iter().enumerate() {
            res.add(idx, v);
        }
        res
    }
}
