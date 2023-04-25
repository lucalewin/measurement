use crate::{Quantity, Dimension};

pub type Time = Quantity<Dimension<1, 0, 0>>;
pub type Length = Quantity<Dimension<0, 1, 0>>;
pub type Mass = Quantity<Dimension<0, 0, 1>>;

pub type Velocity = Quantity<Dimension<-1, 1, 0>>;
pub type Acceleration = Quantity<Dimension<-2, 1, 0>>;
