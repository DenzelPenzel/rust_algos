use std::hint::spin_loop;
use std::ops::{Deref, DerefMut};
use std::{cell::UnsafeCell, sync::atomic::AtomicBool};

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock<'a>(&'a self) -> SpinLockGuard<'a, T> {
        // we try to atomically change false to true
        while self
            .locked
            .compare_exchange(
                false,
                true,
                std::sync::atomic::Ordering::Acquire,
                std::sync::atomic::Ordering::Relaxed,
            )
            .is_err()
        {
            // it hints to the CPU that the thread is busy-waiting
            // it does not put the thread to sleep
            spin_loop();
        }

        SpinLockGuard {
            locked: &self.locked,
            value: unsafe { &mut *self.value.get() },
        }
    }
}

pub struct SpinLockGuard<'a, T> {
    locked: &'a AtomicBool,
    value: &'a mut T,
}

//
impl<T> Deref for SpinLockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<T> DerefMut for SpinLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value
    }
}

impl<T> Drop for SpinLockGuard<'_, T> {
    fn drop(&mut self) {
        self.locked
            .store(false, std::sync::atomic::Ordering::Release);
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
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
