# Measurement

Adds quantities and units to rust to allow unit safe computation with zero cost abstraction.

**Note**: this crate uses an unstable feature which requires you to add `#![feature(generic_const_exprs)]` and use the nightly compiler.

## How to use

Add the `generic_const_exprs` feature by adding `#![feature(generic_const_exprs)]` to your `main.rs` or `lib.rs` file.

```rust
use measurement::si::{Length, Time, Velocity};

fn main() {
    let length = Length::new(20.0); // 20m
    let time = Time::new(5.0); // 5s
    
    // you can specify the type of the quantity but you don't have to
    // (see the example below)
    let velocity: Velocity = length / time;

    println!("{}", velocity.value);
}
```

```rust
use measurement::si::{Mass, Acceleration};

fn main() {
    let mass = Mass::new(50.0);
    let acceleration = Acceleration::new(10.0);

    let force = mass * acceleration;

    println!("{}", force.value);
}
```

## Define new quantities

currently only three dimensions are supported: Time, Length, Mass

```rust
use measurement::{Quantity, Dimension};

pub type Length                  = Quantity<Dimension<1, 0, 0, 0, 0, 0, 0>>;
pub type Mass                    = Quantity<Dimension<0, 1, 0, 0, 0, 0, 0>>;
pub type Time                    = Quantity<Dimension<0, 0, 1, 0, 0, 0, 0>>;
pub type ElectricalCurrent       = Quantity<Dimension<0, 0, 0, 1, 0, 0, 0>>;
pub type ThermodynamicTemperatur = Quantity<Dimension<0, 0, 0, 0, 1, 0, 0>>;
pub type AmountOfSubstance       = Quantity<Dimension<0, 0, 0, 0, 0, 1, 0>>;
pub type LuminousIntensity       = Quantity<Dimension<0, 0, 0, 0, 0, 0, 1>>;

pub type Velocity     = Quantity<Dimension<1, 0, -1, 0, 0, 0, 0>>;
pub type Acceleration = Quantity<Dimension<1, 0, -2, 0, 0, 0, 0>>;
pub type Force        = Quantity<Dimension<1, 1, -2, 0, 0, 0, 0>>;

```
