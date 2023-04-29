use core::ops::Add;

use typenum::{Integer, Sum, NonZero, Unsigned, NInt, PInt};

use crate::types::{TArray, TypeArray, Terminator};

impl Add for Terminator {
    type Output = Self;
    
    fn add(self, _: Self) -> Self::Output {
        Self::Output {}
    }
}

impl<F1: Integer, T1, F2: Integer, T2> Add<TArray<F2, T2>> for TArray<F1, T1>
where
    F1: Integer + Add<F2>,
    T1: Add<T2>,
    T1: TypeArray,
    T2: TypeArray
{
    type Output = TArray<Sum<F1, F2>, Sum<T1, T2>>;

    fn add(self, _: TArray<F2, T2>) -> Self::Output {
        Self::Output::new()
    }
}

impl Add<TArray<typenum::Z0, Terminator>> for Terminator {
    type Output = Terminator;

    fn add(self, _rhs: TArray<typenum::Z0, Terminator>) -> Self::Output {
        Terminator
    }
}

impl<U: Unsigned + NonZero> Add<TArray<typenum::PInt<U>, Terminator>> for Terminator {
    type Output = TArray<typenum::PInt<U>, Terminator>;

    fn add(self, _rhs: TArray<typenum::PInt<U>, Terminator>) -> Self::Output {
        Self::Output::new()
    }
}

impl<U: Unsigned + NonZero> Add<TArray<typenum::NInt<U>, Terminator>> for Terminator {
    type Output = TArray<typenum::NInt<U>, Terminator>;

    fn add(self, _rhs: TArray<typenum::NInt<U>, Terminator>) -> Self::Output {
        Self::Output::new()
    }
}

impl Add<Terminator> for TArray<typenum::Z0, Terminator> {
    type Output = Terminator;

    fn add(self, _rhs: Terminator) -> Self::Output {
        Terminator
    }
}

impl<U: Unsigned + NonZero> Add<Terminator> for TArray<PInt<U>, Terminator> {
    type Output = TArray<PInt<U>, Terminator>;

    fn add(self, _rhs: Terminator) -> Self::Output {
        Self::Output::new()
    }
}

impl<U: Unsigned + NonZero> Add<Terminator> for TArray<NInt<U>, Terminator> {
    type Output = TArray<NInt<U>, Terminator>;

    fn add(self, _rhs: Terminator) -> Self::Output {
        Self::Output::new()
    }
}

impl<N: Integer, N2, T2: TypeArray> Add<TArray<N, TArray<N2, T2>>> for Terminator
where
    TArray<N2, T2>: Add<Terminator>,
    Sum<TArray<N2, T2>, Terminator>: TypeArray
{
    type Output = Sum<TArray<N2, T2>, Terminator>; // FIXME: this does not work

    fn add(self, _rhs: TArray<N, TArray<N2, T2>>) -> Self::Output {
        Self::Output::new()
    }
}
