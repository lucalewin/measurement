# Measurement

Adds quantities and units to rust to allow unit safe computation

**Note**: this crate uses an unstable feature which requires you to add `#![feature(generic_const_exprs)]` and use the nightly compiler

## Define new quantity

currently only three dimensions are supported: Time, Length, Mass

```rust
use measurement::{Quantity, Dimension};

pub type Time = Quantity<Dimension<1 /* time */, 0 /* length */, 0 /* mass */>>;
pub type Length = Quantity<Dimension<0, 1, 0>>;
pub type Mass = Quantity<Dimension<0, 0, 1>>;

pub type Velocity = Quantity<Dimension<-1, 1, 0>>;
pub type Acceleration = Quantity<Dimension<-2, 1, 0>>;

```
