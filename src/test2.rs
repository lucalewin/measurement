#![allow(dead_code)]

use std::marker::PhantomData;
use num::Num;

trait Unit {}
trait Dimension {}
// trait Dimension<Time, Length, Mass, Current, Temperature, Substance, Luminous> {}

struct Dim {}
impl Dimension for Dim {}

struct Quantity<D, U, V = f64>
where
    D: Dimension,
    U: Unit,
    V: Num
{
    pub dimension: PhantomData<D>,
    pub unit: PhantomData<U>,
    pub value: V
}

impl<D, U, V> Quantity<D, U, V>
where
    D: Dimension,
    U: Unit,
    V: Num
{
    pub fn new(value: V) -> Self {
        Self {
            dimension: PhantomData,
            unit: PhantomData,
            value,
        }
    }
}

impl<D, U, V> std::ops::Add for Quantity<D, U, V>
where 
    D: Dimension,
    U: Unit,
    V: Num
{
    type Output = Quantity<D, U, V>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            dimension: PhantomData,
            unit: PhantomData,
            value: self.value + rhs.value
        }
    }
}
impl<D, U, V> std::ops::AddAssign for Quantity<D, U, V>
where 
    D: Dimension,
    U: Unit,
    V: Num + std::ops::AddAssign
{
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}
impl<D, U, V> std::ops::Sub for Quantity<D, U, V>
where 
    D: Dimension,
    U: Unit,
    V: Num
{
    type Output = Quantity<D, U, V>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            dimension: PhantomData,
            unit: PhantomData,
            value: self.value - rhs.value
        }
    }
}
impl<D, U, V> std::ops::SubAssign for Quantity<D, U, V>
where 
    D: Dimension,
    U: Unit,
    V: Num + std::ops::SubAssign
{
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

// Length
struct Meter {}
struct Second {}
struct MeterPerSecond{}

impl Unit for Meter {}
impl Unit for Second {}
impl Unit for MeterPerSecond {}

type Length<U = Meter> = Quantity<Dim, U>;
type Time = Quantity<Dim, Second>;
type Velocity = Quantity<Dim, MeterPerSecond>;

#[cfg(test)]
mod tests {
    use super::{Length, Time, Velocity, Second};

    #[test]
    fn test() {
        let length: Length = Length::new(20.0);
        let time = Time::new(5.0);
        let time2 = Time::new(5.0);

        let time3 = time + time2;

        // let velocity = speed(length, time);
    }

    #[test]
    fn different_unit() {
        let test: Length<Second> = Length::new(2.0);
    }

    fn speed(length: Length, time: Time) -> Velocity {
        todo!()
    }
}
