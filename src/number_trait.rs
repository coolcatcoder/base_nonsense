use std::ops::{AddAssign, DivAssign, MulAssign, Rem};

use num_bigint::BigUint;

pub trait Number:
    Sized + Clone + DivAssign<Self::Rhs> + MulAssign<Self::Rhs> + AddAssign<Self::Rhs>
{
    const ZERO: Self;

    type Rhs: Copy;
    fn rhs_from_usize(value: usize) -> Self::Rhs;
    fn rhs_to_usize(value: Self::Rhs) -> usize;

    fn not_zero(&self) -> bool;

    fn remainder(&self, rhs: Self::Rhs) -> Self::Rhs;
}
impl Number for u32 {
    const ZERO: Self = 0;

    type Rhs = Self;
    fn rhs_from_usize(value: usize) -> Self::Rhs {
        value.strict_cast()
    }
    fn rhs_to_usize(value: Self::Rhs) -> usize {
        value.strict_cast()
    }

    fn not_zero(&self) -> bool {
        *self != 0
    }

    fn remainder(&self, rhs: Self::Rhs) -> Self {
        self.rem(rhs)
    }
}

impl Number for BigUint {
    const ZERO: Self = Self::ZERO;

    type Rhs = u32;
    fn rhs_from_usize(value: usize) -> Self::Rhs {
        value.strict_cast()
    }
    fn rhs_to_usize(value: Self::Rhs) -> usize {
        value.strict_cast()
    }

    fn not_zero(&self) -> bool {
        self != &Self::ZERO
    }

    fn remainder(&self, rhs: Self::Rhs) -> Self::Rhs {
        self.rem(rhs).try_into().unwrap()
    }
}
