use crate::quantity;

// base units
quantity!(
    name: Time,
    unit: Second
);
quantity!(
    name: Length,
    unit: Meter
);
quantity!(
    name: Mass,
    unit: Kilogram
);

// derived
quantity!(
    name: Velocity,
    unit: MeterPerSecond,
    derive: Length / Time
);
quantity!(
    name: Acceleration,
    unit: MeterPerSquareSecond,
    derive: Velocity / Time
);
quantity!(
    name: Force,
    unit: Newton,
    derive: Mass * Acceleration
);
