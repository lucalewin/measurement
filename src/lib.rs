//!
//! 
//! Order of base quantities:
//! 1. Length [m]
//! 2. Mass [kg]
//! 3. Time [s]
//! 4. Electrical Current [A]
//! 5. Thermodynamic Temperature [K]
//! 6. Amount of Substance [mol]
//! 7. Luminous Intensity [cd]
//! 

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod si;

use std::marker::PhantomData;

#[derive(Debug, Copy, Clone)]
pub struct Dimension<
    const LENGTH: isize,
    const MASS: isize,
    const TIME: isize,
    const ELEICAL_CURRENT: isize,
    const THERMODYNAMIC_TEMPERATURE: isize,
    const AMOUNT_OF_SUBSTANCE: isize,
    const LUMINOUS_INTENSITY: isize,
>;

trait SiDim {}
impl<
    const L: isize,
    const M: isize,
    const T: isize,
    const I: isize,
    const TT: isize,
    const N: isize,
    const J: isize
> SiDim for Dimension<L, M, T, I, TT, N, J> {}

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct Quantity<D> {
    pub dim: PhantomData<D>,
    pub value: f64
}

impl<
    const L: isize,
    const M: isize,
    const T: isize,
    const I: isize,
    const TT: isize,
    const N: isize,
    const J: isize
> Quantity<Dimension<L, M, T, I, TT, N, J>> {
    pub const fn new(value: f64) -> Self {
        Self {
            dim: PhantomData,
            value
        }
    }
}

impl<
    const L: isize,
    const M: isize,
    const T: isize,
    const I: isize,
    const TT: isize,
    const N: isize,
    const J: isize
> std::ops::Add for Quantity<Dimension<L, M, T, I, TT, N, J>> {
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            dim: PhantomData,
            value: self.value + rhs.value
        }
    }
}

impl<
    const L: isize,
    const M: isize,
    const T: isize,
    const I: isize,
    const TT: isize,
    const N: isize,
    const J: isize
> std::ops::Sub for Quantity<Dimension<L, M, T, I, TT, N, J>> {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            dim: PhantomData,
            value: self.value - rhs.value
        }
    }
}

impl<
    const L1: isize,
    const M1: isize,
    const T1: isize,
    const I1: isize,
    const TT1: isize,
    const N1: isize,
    const J1: isize,

    const L2: isize,
    const M2: isize,
    const T2: isize,
    const I2: isize,
    const TT2: isize,
    const N2: isize,
    const J2: isize
> std::ops::Mul<Quantity<Dimension<L2, M2, T2, I2, TT2, N2, J2>>> for Quantity<Dimension<L1, M1, T1, I1, TT1, N1, J1>>
where
    Quantity<Dimension<{L1 + L2}, {M1 + M2}, {T1 + T2}, {I1 + I2}, {TT1 + TT2}, {N1 + N2}, {J1 + J2}>>: Sized
{
    type Output = Quantity<Dimension<{L1 + L2}, {M1 + M2}, {T1 + T2}, {I1 + I2}, {TT1 + TT2}, {N1 + N2}, {J1 + J2}>>;

    fn mul(self, rhs: Quantity<Dimension<L2, M2, T2, I2, TT2, N2, J2>>) -> Self::Output {
        Self::Output {
            dim: PhantomData,
            value: self.value * rhs.value
        }
    }
}

impl<
    const L1: isize,
    const M1: isize,
    const T1: isize,
    const I1: isize,
    const TT1: isize,
    const N1: isize,
    const J1: isize,

    const L2: isize,
    const M2: isize,
    const T2: isize,
    const I2: isize,
    const TT2: isize,
    const N2: isize,
    const J2: isize
> std::ops::Div<Quantity<Dimension<L2, M2, T2, I2, TT2, N2, J2>>> for Quantity<Dimension<L1, M1, T1, I1, TT1, N1, J1>>
where
    Quantity<Dimension<{L1 - L2}, {M1 - M2}, {T1 - T2}, {I1 - I2}, {TT1 - TT2}, {N1 - N2}, {J1 - J2}>>: Sized
{
    type Output = Quantity<Dimension<{L1 - L2}, {M1 - M2}, {T1 - T2}, {I1 - I2}, {TT1 - TT2}, {N1 - N2}, {J1 - J2}>>;

    fn div(self, rhs: Quantity<Dimension<L2, M2, T2, I2, TT2, N2, J2>>) -> Self::Output {
        Self::Output {
            dim: PhantomData,
            value: self.value / rhs.value
        }
    }
}
