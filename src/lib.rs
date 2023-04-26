#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod si;

use std::marker::PhantomData;

#[derive(Debug, Copy, Clone)]
pub struct Dimension<
    const TIME: isize,
    const LENGTH: isize,
    const MASS: isize,
>;

trait SiDim {}
impl<
    const T: isize,
    const L: isize,
    const M: isize
> SiDim for Dimension<T, L, M> {}

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct Quantity<D> {
    pub dim: PhantomData<D>,
    pub value: f64
}

impl<
    const T: isize,
    const L: isize,
    const M: isize
> Quantity<Dimension<T, L, M>> {
    pub const fn new(value: f64) -> Self {
        Self {
            dim: PhantomData,
            value
        }
    }
}

impl<
    const T: isize,
    const L: isize,
    const M: isize
> std::ops::Add for Quantity<Dimension<T, L, M>> {
    type Output = Quantity<Dimension<T, L, M>>;
    
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            dim: PhantomData,
            value: self.value + rhs.value
        }
    }
}

impl<
    const T: isize,
    const L: isize,
    const M: isize
> std::ops::Sub for Quantity<Dimension<T, L, M>> {
    type Output = Quantity<Dimension<T, L, M>>;
    
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            dim: PhantomData,
            value: self.value - rhs.value
        }
    }
}

impl<
    const T1: isize,
    const L1: isize,
    const M1: isize,
    const T2: isize,
    const L2: isize,
    const M2: isize
> std::ops::Mul<Quantity<Dimension<T2, L2, M2>>> for Quantity<Dimension<T1, L1, M1>>
where
    Quantity<Dimension<{T1 + T2}, {L1 + L2}, {M1 + M2}>>: Sized
{
    type Output = Quantity<Dimension<{T1 + T2}, {L1 + L2}, {M1 + M2}>>;

    fn mul(self, rhs: Quantity<Dimension<T2, L2, M2>>) -> Self::Output {
        Self::Output {
            dim: PhantomData,
            value: self.value * rhs.value
        }
    }
}

impl<
    const T1: isize,
    const L1: isize,
    const M1: isize,
    const T2: isize,
    const L2: isize,
    const M2: isize
> std::ops::Div<Quantity<Dimension<T2, L2, M2>>> for Quantity<Dimension<T1, L1, M1>>
where
    Quantity<Dimension<{T1 - T2}, {L1 - L2}, {M1 - M2}>>: Sized
{
    type Output = Quantity<Dimension<{T1 - T2}, {L1 - L2}, {M1 - M2}>>;

    fn div(self, rhs: Quantity<Dimension<T2, L2, M2>>) -> Self::Output {
        Self::Output {
            dim: PhantomData,
            value: self.value / rhs.value
        }
    }
}
