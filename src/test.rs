#![allow(unused)]

extern crate std;

use core::marker::PhantomData;
use std::ops::Add;

use typenum::{consts::{N2, N1, Z0, P1, P2}, Sum};

use crate::{types::{TArray, Terminator, TypeArray}, tarr};

trait DimensionTrait {}
struct Quantity<D: DimensionTrait> {
    d: PhantomData<D>,
    pub value: f64
}

struct Dimension<T: TypeArray> {
    t: PhantomData<T>
}

impl<T: TypeArray> DimensionTrait for Dimension<T> {}

// --------------------------------------------------------------

impl<D: DimensionTrait> Quantity<D> {
    pub const fn new(value: f64) -> Self {
        Self {
            d: PhantomData,
            value
        }
    }
}

impl<D: DimensionTrait> core::ops::Add for Quantity<D> {
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            d: PhantomData,
            value: self.value + rhs.value
        }
    }
}

impl<T1, T2> core::ops::Mul<Quantity<Dimension<T2>>> for Quantity<Dimension<T1>>
where
    T1: TypeArray + Add<T2>,
    T2: TypeArray,
    Sum<T1, T2>: TypeArray
{
    type Output = Quantity<Dimension<Sum<T1, T2>>>;
    
    fn mul(self, rhs: Quantity<Dimension<T2>>) -> Self::Output {
        Self::Output {
            d: PhantomData,
            value: self.value * rhs.value
        }
    }
}

// --------------------------------------------------------------

type Length         = Quantity<Dimension<tarr![P1]>>;
type Mass           = Quantity<Dimension<tarr![Z0, P1]>>;
type Time           = Quantity<Dimension<tarr![Z0, Z0, P1]>>;

type Velocity       = Quantity<Dimension<tarr![P1, Z0, N1]>>;
type Acceleration   = Quantity<Dimension<tarr![P1, Z0, N2]>>;
type Force          = Quantity<Dimension<tarr![P1, P1, N2]>>;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add() {
        let l1 = Length::new(20.0);
        let l2 = Length::new(30.0);
        
        let l3: Length = l1 + l2;
        
        assert_eq!(l3.value, 50.0);
    }
    
    #[test]
    fn sub() {
        
    }
    
    #[test]
    fn mul() {
        // let v = Velocity::new(5.0);
        // let t = Time::new(4.0);
        
        // let l: Length = t * v;
        
        // assert_eq!(l.value, 20.0);
        
        // let l = Length::new(3.0);
        // let m = Mass::new(2.0);
        
        // let idk: Quantity<Dimension<TArray<P1, TArray<P1, Terminator>>>> = m * l;
    }
    
    #[test]
    fn div() {
        
    }
}

// trait DimensionTrait {}
// struct Quantity<D: DimensionTrait> {
//     d: PhantomData<D>,
//     value: f64
// }
// use typenum::*;
// struct Dimension<Exps: TypeArray> {
//     p: PhantomData<Exps>
// }
// impl<TA: TypeArray> DimensionTrait for Dimension<TA> {}

// type Length = Quantity<Dimension<tarr![P1]>>;
// type Mass = Quantity<Dimension<tarr![Z0, P1]>>;
// type Time = Quantity<Dimension<tarr![Z0, Z0, P1]>>;

// type Velocity = Quantity<Dimension<tarr![P1, Z0, N1]>>;
// type Acceleration = Quantity<Dimension<tarr![P1, Z0, N2]>>;
// type Force = Quantity<Dimension<tarr![P1, P1, N2]>>;

