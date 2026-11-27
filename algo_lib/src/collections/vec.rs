#[cfg(test)]
mod tests {
    use super::Vec;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn empty_vector_can_be_read_and_popped() {
        let mut values = Vec::<i32>::new();
        assert!(values.is_empty());
        assert_eq!(values.len(), 0);
        assert_eq!(values.capacity(), 0);
        assert_eq!(values.as_slice(), &[]);
        assert_eq!(values.get(0), None);
        assert_eq!(values.pop(), None);
    }

    #[test]
    fn growth_preserves_owned_values_and_pop_returns_them_in_reverse() {
        let mut values = Vec::with_capacity(2);
        for i in 0..100 {
            values.push(i.to_string());
        }
        assert_eq!(values.len(), 100);
        assert!(values.capacity() >= 100);
        for i in (0..100).rev() {
            assert_eq!(values.pop(), Some(i.to_string()));
        }
        assert_eq!(values.pop(), None);
        values.push(String::from("reused"));
        assert_eq!(values[0], "reused");
    }

    #[test]
    fn access_checks_length_and_allows_mutation() {
        let mut values = Vec::with_capacity(8);
        values.push(10);
        values.push(20);
        values[0] = 11;
        *values.get_mut(1).unwrap() = 21;
        assert_eq!(values.as_slice(), &[11, 21]);
        assert_eq!(values.get(2), None);
        assert_eq!(values.get_mut(2), None);
        assert_eq!(values.get(usize::MAX), None);
        assert_eq!(values.capacity(), 8);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn indexing_spare_capacity_panics() {
        let mut values = Vec::with_capacity(8);
        values.push(10);
        let _ = values[1];
    }

    struct Tracked(Rc<Cell<usize>>);

    impl Drop for Tracked {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    #[test]
    fn elements_are_dropped_exactly_once() {
        let drops = Rc::new(Cell::new(0));
        let mut values = Vec::new();
        for _ in 0..10 {
            values.push(Tracked(Rc::clone(&drops)));
        }
        assert_eq!(drops.get(), 0);
        let popped = values.pop();
        drop(values);
        assert_eq!(drops.get(), 9);
        drop(popped);
        assert_eq!(drops.get(), 10);
    }

    #[test]
    fn zero_sized_values_support_access_and_pop() {
        let mut values = Vec::new();
        for _ in 0..100 {
            values.push(());
        }
        assert_eq!(values.len(), 100);
        assert_eq!(values.capacity(), usize::MAX);
        assert_eq!(values.get(99), Some(&()));
        assert_eq!(values.as_mut_slice().len(), 100);
        for _ in 0..100 {
            assert_eq!(values.pop(), Some(()));
        }
        assert_eq!(values.pop(), None);
    }

    #[test]
    fn zero_sized_values_are_dropped() {
        thread_local! {
            static DROPS: Cell<usize> = const { Cell::new(0) };
        }
        struct ZeroSized;
        impl Drop for ZeroSized {
            fn drop(&mut self) {
                DROPS.with(|drops| drops.set(drops.get() + 1));
            }
        }
        DROPS.with(|drops| drops.set(0));
        let mut values = Vec::with_capacity(4);
        values.push(ZeroSized);
        values.push(ZeroSized);
        drop(values.pop());
        drop(values);
        DROPS.with(|drops| assert_eq!(drops.get(), 2));
    }

    #[test]
    fn allocation_preserves_alignment_when_growing() {
        #[repr(align(64))]
        struct Aligned(u8);
        let mut values = Vec::new();
        for i in 0..10 {
            values.push(Aligned(i));
            assert_eq!(values.as_slice().as_ptr() as usize % 64, 0);
        }
        for i in 0..10 {
            assert_eq!(values[i].0, i as u8);
        }
    }

    #[test]
    #[should_panic(expected = "capacity overflow")]
    fn impossible_capacity_panics_before_allocation() {
        let _ = Vec::<u64>::with_capacity(usize::MAX);
    }
}
