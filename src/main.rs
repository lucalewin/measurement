// use std::marker::PhantomData;

// use measurement::test::{Quantity, Length, Time, Meter, Second, Velocity, MeterPerSecond};

use measurement::si::{Length, Time, Velocity, Mass};

fn main() {

    // let length = Quantity::<Length<Meter>> { value: 15.0, d: PhantomData::default() };
    // let time = Quantity::<Time<Second>> { value: 5.0, d: PhantomData::default() };
    
    // let acceleration = length / time;

    // println!("acc: {}", acceleration.value);

    // println!("size: {}", core::mem::size_of::<Quantity<Velocity<MeterPerSecond>>>());
    test()
}

fn test() {
    let length = Length::new(20.0);
    let time = Time::new(5.0);
    let velocity = speed(length, time);

    let acc = velocity / Time::new(2.0);

    let force = Mass::new(30.0) * acc;

    println!("force: {}", force.0);
}

fn speed(length: Length, time: Time) -> Velocity {
    length / time
}
