use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}

pub trait AdditionMonoid: Add<Output = Self> + AddAssign + Zero + Eq + Sized {}

impl<T: Add<Output = Self> + AddAssign + Zero + Eq> AdditionMonoid for T {}

pub trait AdditionMonoidWithSub: AdditionMonoid + Sub<Output = Self> + SubAssign {}

impl<T: AdditionMonoid + Sub<Output = Self> + SubAssign> AdditionMonoidWithSub for T {}
