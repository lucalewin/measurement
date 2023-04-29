use std::marker::PhantomData;
use core::ops::Add;

use typenum::{Integer, Sum, Unsigned, NonZero};

pub trait TypeArray {
    fn new() -> Self;
}

pub struct TArray<F, T> {
    first: PhantomData<F>,
    tail: PhantomData<T>
}
pub struct Terminator;

impl<F, T> TypeArray for TArray<F, T> {
    fn new() -> Self {
        Self { first: PhantomData, tail: PhantomData }
    }
}
impl TypeArray for Terminator {
    fn new() -> Self {
        Terminator
    }
}

// impl<F, T> TArray<F, T> {
//     pub const fn new() -> Self {
//         Self { first: PhantomData, tail: PhantomData }
//     }
// }

#[macro_export]
macro_rules! tarr {
    () => ( $crate::types::Terminator );
    ($n:ty) => ( $crate::types::TArray<$n, $crate::types::Terminator> );
    ($n:ty, $($tail:ty),+) => ( $crate::types::TArray<$n, tarr![$($tail),+]> );
}

#[cfg(test)]
mod tests {
    use std::{marker::PhantomData, ops::Add};

    use typenum::{Z0, P1, Sum, P10, N1, N10};

    use super::Terminator;

    struct Wrapper<T> { t: PhantomData<T> }

    impl<T> Wrapper<T> {
        pub fn new() -> Self { Self { t: PhantomData } }
    }

    impl<T1, T2> Add<Wrapper<T2>> for Wrapper<T1>
    where
        T1: Add<T2>
    {
        type Output = Wrapper<Sum<T1, T2>>;

        fn add(self, _rhs: Wrapper<T2>) -> Self::Output {
            Self::Output::new()
        }
    }

    #[test]
    fn terminator() {
        let _: Terminator = Terminator {} + Terminator {};
    }

    #[test]
    fn single() {
        let term = Wrapper::<Terminator>::new();
        let arr = Wrapper::<tarr![Z0]>::new();

        let _sum: Wrapper<Terminator> = term + arr;

        // ---------------------------

        let term = Wrapper::<Terminator>::new();
        let arr = Wrapper::<tarr![P1]>::new();

        let _sum: Wrapper<tarr![P1]> = term + arr;
        
        // ---------------------------

        let term = Wrapper::<Terminator>::new();
        let arr = Wrapper::<tarr![P10]>::new();

        let _sum: Wrapper<tarr![P10]> = term + arr;

        // ---------------------------

        let term = Wrapper::<Terminator>::new();
        let arr = Wrapper::<tarr![N1]>::new();

        let _sum: Wrapper<tarr![N1]> = term + arr;
        
        // ---------------------------

        let term = Wrapper::<Terminator>::new();
        let arr = Wrapper::<tarr![N10]>::new();

        let _sum: Wrapper<tarr![N10]> = term + arr;
    }

    #[test]
    fn reverse() {
        let term = Wrapper::<Terminator>::new();
        let arr = Wrapper::<tarr![Z0]>::new();

        let _sum: Wrapper<Terminator> = arr + term;

        // ---------------------------

        let term = Wrapper::<Terminator>::new();
        let arr = Wrapper::<tarr![P1]>::new();

        let _sum: Wrapper<tarr![P1]> = arr + term;
        
        // ---------------------------

        let term = Wrapper::<Terminator>::new();
        let arr = Wrapper::<tarr![P10]>::new();

        let _sum: Wrapper<tarr![P10]> = arr + term;

        // ---------------------------

        let term = Wrapper::<Terminator>::new();
        let arr = Wrapper::<tarr![N1]>::new();

        let _sum: Wrapper<tarr![N1]> = arr + term;
        
        // ---------------------------

        let term = Wrapper::<Terminator>::new();
        let arr = Wrapper::<tarr![N10]>::new();

        let _sum: Wrapper<tarr![N10]> = arr + term;
    }

    #[test]
    fn double_positive() {
        let term = Wrapper::<Terminator>::new();
        let arr = Wrapper::<tarr![Z0, P1]>::new();

        let _sum: Wrapper<tarr![Z0, P1]> = term + arr;
    }

    #[test]
    fn double_zero() {
        let term = Wrapper::<Terminator>::new();
        let arr = Wrapper::<tarr![P1, Z0]>::new();

        let _sum: Wrapper<tarr![P1]> = term + arr;
    }

    #[test]
    fn same_size() {
        let arr1 = Wrapper::<tarr![P1, Z0, N1]>::new();
        let arr2 = Wrapper::<tarr![Z0, Z0, P1]>::new();

        let _sum: Wrapper::<tarr![P1]> = arr1 + arr2;

        // ---------------------------------

        let arr1 = Wrapper::<tarr![P1, Z0, Z0]>::new();
        let arr2 = Wrapper::<tarr![N1, Z0, P1]>::new();

        let _sum: Wrapper::<tarr![Z0, Z0, P1]> = arr1 + arr2;
    }
}
