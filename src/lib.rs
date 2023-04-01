#![macro_use]

pub mod si;

pub trait Quantity {}
pub trait Unit {}

#[macro_export]
macro_rules! quantity {
    // rule for a base quantity
    (
        name: $qty:ident,
        base_unit: $base_unit:ident,
        units: { $($unit_name:ident: $factor:expr,)+ }
    ) => {
        // quantity + unit
        $crate::impl_quantity_and_unit!($qty, $base_unit);
    };
    (
        name: $qty:ident,
        base_unit: $base_unit:ident,
        units: { $($unit_name:ident: $factor:expr,)+ },
        derive: $lhs:ident $op:tt $rhs:ident $(,)?
    ) => {
        $crate::impl_quantity_and_unit!($qty, $base_unit);
        $crate::impl_ops!($qty, $lhs $op $rhs);
    }
}

#[macro_export]
macro_rules! impl_quantity_and_unit {
    ($quantity_name:ident, $unit_name:ident) => {
        $crate::impl_unit!($unit_name, $quantity_name);
        $crate::impl_quantity!($quantity_name, $unit_name);
    };
}

#[macro_export]
macro_rules! impl_quantity {
    ($quantity_name:ident, $base_unit_ident:ident) => {
        pub struct $quantity_name
        {
            value: f64,
        }
        impl $crate::Quantity for $quantity_name {}
        impl $quantity_name {
            #[inline(always)]
            pub fn new(value: f64) -> $quantity_name {
                $quantity_name { value }
            }
            #[inline(always)]
            pub fn value(&self) -> f64 {
                self.value
            }
        }
    };
}

#[macro_export]
macro_rules! impl_unit {
    ($unit_name:ident, $qty_name:ident) => {

        pub struct $unit_name;
        impl $crate::Unit for $unit_name {}
    };
}

#[macro_export]
macro_rules! impl_ops {
    ($qty:ident, $lhs:ident * $rhs:ident) => {
        $crate::impl_mul!($lhs * $rhs = $qty);
        $crate::impl_div!($qty / $rhs = $lhs);
        $crate::impl_div!($qty / $lhs = $rhs);
    };
    ($qty:ident, $lhs:ident / $rhs:ident) => {
        $crate::impl_div!($lhs / $rhs = $qty);
        $crate::impl_div!($lhs / $qty = $rhs);
        $crate::impl_mul!($qty * $rhs = $lhs);
    };
}

#[macro_export]
macro_rules! impl_mul {
    ($lhs:ident * $rhs:ident = $out:ident) => {
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = $out;
            
            fn mul(self, rhs: $rhs) -> Self::Output {
                $out {
                    value: self.value * rhs.value
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_div {
    ($lhs:ident / $rhs:ident = $out:ident) => {
        impl std::ops::Div<$rhs> for $lhs {
            type Output = $out;

            fn div(self, rhs: $rhs) -> Self::Output {
                $out{
                    value: self.value / rhs.value
                }
            }
        }
    };
}
