#![macro_use]

pub mod test;
pub mod si;

pub trait Quantity {}
pub trait Unit {}

#[macro_export]
macro_rules! quantity {
    // rule for a base quantity
    (name: $n:ident, unit: $u:ident) => {
        $crate::impl_quantity_and_unit!($n, $u);
    };

    // derived quantity (derived by multiplication)
    (name: $n:ident, unit: $u:ident, derive: $lhs:ident * $rhs:ident) => {
        $crate::impl_quantity_and_unit!($n, $u);
        
        // operators
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = $n;
            
            fn mul(self, rhs: $rhs) -> Self::Output {
                $n(self.0 * rhs.0)
            }
        }
    };

    // derived quantity (derived by division)
    (name: $n:ident, unit: $u:ident, derive: $lhs:ident / $rhs:ident) => {
        $crate::impl_quantity_and_unit!($n, $u);

        // operators
        impl std::ops::Div<$rhs> for $lhs {
            type Output = $n;

            fn div(self, rhs: $rhs) -> Self::Output {
                $n(self.0 / rhs.0)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_quantity_and_unit {
    ($quantity_name:ident, $unit_name:ident) => {
        pub struct $quantity_name(pub f64);
        impl $crate::Quantity for $quantity_name {}
        impl $quantity_name {
            pub fn new(val: f64) -> Self {
                Self(val)
            }
        }
        // unit
        pub struct $unit_name;
        impl $crate::Unit for $unit_name {}
    };
}
