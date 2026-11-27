use std::{
    alloc::{Layout, alloc, dealloc, handle_alloc_error, realloc},
    ptr::{self, NonNull},
    usize,
};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub struct Vector<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            cap: if size_of::<T>() == 0 { usize::MAX } else { 0 },
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, value: T) {
        if self.len() == self.cap {
            self.grow()
        }

        let base = self.ptr.as_ptr();
        // new point to the next free cell
        let dest = unsafe { base.add(self.len) };
        // write new value to this new cell
        unsafe {
            dest.write(value);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.len -= 1;
        unsafe { Some(self.ptr.as_ptr().add(self.len).read()) }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len() {
            return None;
        }
        unsafe { Some(&*self.ptr.as_ptr().add(index)) }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len() {
            return None;
        }
        unsafe { Some(&mut *self.ptr.as_ptr().add(index)) }
    }

    fn grow(&mut self) {
        assert!(size_of::<T>() != 0, "capacity overflow");

        let new_cap = if self.cap == 0 {
            1
        } else {
            self.cap.checked_mul(2).expect("capacity overflow")
        };

        let new_layout = Layout::array::<T>(new_cap).expect("capacity overflow");
        let raw = unsafe {
            if self.cap == 0 {
                alloc(new_layout)
            } else {
                let old_layout = Layout::array::<T>(self.cap).expect("valid capacity");

                realloc(self.ptr.as_ptr().cast(), old_layout, new_layout.size())
            }
        };

        let new_ptr = match NonNull::new(raw.cast::<T>()) {
            Some(ptr) => ptr,
            None => handle_alloc_error(new_layout),
        };

        self.ptr = new_ptr;
        self.cap = new_cap;
    }
}

impl<T> Default for Vector<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.ptr.as_ptr(), self.len));
        }

        if size_of::<T>() != 0 && self.cap != 0 {
            let layout = Layout::array::<T>(self.cap).expect("valid capacity");

            unsafe {
                dealloc(self.ptr.as_ptr().cast(), layout);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
