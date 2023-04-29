#![allow(dead_code)]

use std::marker::PhantomData;
use typenum::*;

// ------------------------------------------------------------

macro_rules! dim {
    ($dim_name:ident, $trait_name:ident, $($param:ident)+) => {
        #[allow(non_snake_case)]
        struct $dim_name<$($param),+>
        where
            $( $param: $trait_name),+
        {
            $( $param: PhantomData<$param>),+
        }
        impl<$($param),+> Dimensions for $dim_name<$($param),+>
        where
            $( $param: $trait_name),+ {}
    };
}

trait Dimen {}
trait Dimensions {}

dim! { Dimension1, Integer, A }
dim! { Dimension2, Integer, A B }
dim! { Dimension3, Integer, A B C }
dim! { Dimension4, Integer, A B C D }

// ------------------------------------------------------------
// TODO: idk
// ------------------------------------------------------------

#[derive(Debug, Default)]
struct Quantity<D: Dimensions> {
    d: PhantomData<D>,
    value: f64
}

impl<D: Dimensions> Quantity<D> {
    pub fn new(value: f64) -> Self {
        Self {
            d: PhantomData,
            value
        }
    }
}

// ------------------------------------------------------------

type Length = Quantity<Dimension3<P1, Z0, Z0>>;
type Time = Quantity<Dimension3<Z0, Z0, P1>>;
type Velocity = Quantity<Dimension3<P1, Z0, N1>>;

#[allow(unused_variables)]
fn main() {
    let length = Length::new(20.0);
}
