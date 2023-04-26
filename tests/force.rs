#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use measurement::si::{Mass, Acceleration, Force};

#[test]
fn test() {
    let mass = Mass::new(50.0);
    let acceleration = Acceleration::new(10.0);

    let force: Force = mass * acceleration;

    assert_eq!(force.value, 500.0);
}