use crate::quantity;

// base units
quantity! {
    name: Time,
    base_unit: Second,
    units: {
        Second:      1e+0,
        Millisecond: 1e-3,
    }
}
quantity! {
    name: Length,
    base_unit: Meter,
    units: {
        Kilometer:  1e+3,
        Meter:      1e+0,
        Centimeter: 1e-2,
        Millimeter: 1e-3,
    }
}
quantity! {
    name: Mass,
    base_unit: Kilogram,
    units: {
        Kilogram:  1e+0,
        Gram:      1e-3,
        Milligram: 1e-6,
    }
}

// derived
quantity! {
    name: Velocity,
    base_unit: MeterPerSecond,
    units: {
        MeterPerSecond: 1,
        KilometerPerHour: 3.6,
    },
    derive: Length / Time,
}
quantity! {
    name: Acceleration,
    base_unit: MeterPerSquareSecond,
    units: {
        MeterPerSquareSecond: 1,
    },
    derive: Velocity / Time,
}
quantity! {
    name: Force,
    base_unit: Newton,
    units: {
        Kilonewton:  1e3,
        Newton:      1,
        Millinewton: 1e-3,
    },
    derive: Mass * Velocity,
}
