use crate::{Quantity, Dimension};

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
