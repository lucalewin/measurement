use std::marker::PhantomData;

use measurement::test::{Quantity, Length, Time, Meter, Second, Velocity, MeterPerSecond};

fn main() {
    let length = Quantity::<Length<Meter>> { value: 15.0, d: PhantomData::default() };
    let time = Quantity::<Time<Second>> { value: 5.0, d: PhantomData::default() };
    
    let acceleration = length / time;

    println!("acc: {}", acceleration.value);

    println!("size: {}", core::mem::size_of::<Quantity<Velocity<MeterPerSecond>>>());
}