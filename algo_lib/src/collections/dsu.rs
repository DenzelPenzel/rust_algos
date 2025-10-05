use std::cell::Cell;
use crate::collections::slice::bounds::Bounds;
use crate::collections::slice::indices::Indices;

#[derive(Clone)]
pub struct DSU {
    // if id[i] is negative, i is a root and -id[i] is the size of the set
    // if id[i] is positive, id[i] is the parent of i
    id: Vec<Cell<i32>>,
    count: usize
}

impl DSU {
    pub fn new(n: usize) -> Self {
        DSU {
            id: vec![Cell::new(-1); n],
            count: n
        }
    }

    pub fn size(&self, i: usize) -> usize {
        (-self.id[self.find(i)].get()) as usize
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.id.len()
    }

    pub fn find(&self, i: usize) -> usize {
        // If id[i] is positive, it's a pointer to the parent
        if self.id[i].get() >= 0 {
            let res = self.find(self.id[i].get() as usize);
            // Path compression: point this node directly to the root
            self.id[i].set(res as i32);
            res
        } else {
            // If id[i] is negative, this is the root
            i
        }
    }

    pub fn iter(&self) -> impl Iterator<Item=usize> + '_ {
        self.id.iter().enumerate().filter_map(|(i, c)| {
            if c.get() < 0 {
                Some(i)
            } else {
                None
            }
        })
    }

    pub fn set_count(&self) -> usize {
        self.count
    }

    pub fn union(&mut self, mut a: usize, mut b: usize) -> bool {
        a = self.find(a);
        b = self.find(b);
        if a == b {
            false
        } else {
            // Merge the smaller set into the larger one (union by size/rank is not implemented here)
            // Update the size of the new root's set
            self.id[a].set(self.id[a].get() + self.id[b].get());
            // Make 'a' the parent of 'b'
            self.id[b].set(a as i32);
            self.count -= 1;
            true
        }
    }

    pub fn clear(&mut self) {
        self.count = self.id.len();
        self.id.fill(Cell::new(-1));
    }

    pub fn parts(&self) -> Vec<Vec<usize>> {
        let roots: Vec<_> = self.iter().collect();
        let mut res = vec![Vec::new(); roots.len()];
        for i in self.id.indices() {
            res[roots.as_slice().bin_search(&self.find(i)).unwrap()].push(i);
        }
        res
    }
}