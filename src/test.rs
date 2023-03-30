use std::{marker::PhantomData, ops::Div};

pub trait Dimension {}
pub trait Unit {}
pub struct Quantity<Dimension> {
    pub value: f64,
    pub d: PhantomData<Dimension>
}

// base quantities
pub struct Time<Unit>(PhantomData<Unit>);
pub struct Length<Unit>(PhantomData<Unit>);
pub struct Mass<Unit>(PhantomData<Unit>);

// derived quantities
pub struct Velocity<Unit>(PhantomData<Unit>);
pub struct Acceleration<Unit>(PhantomData<Unit>);


impl<U: Unit> Dimension for Time<U> {}
impl<U: Unit> Dimension for Length<U> {}
impl<U: Unit> Dimension for Mass<U> {}
impl<U: Unit> Dimension for Velocity<U> {}
impl<U: Unit> Dimension for Acceleration<U> {}


impl Div<Quantity<Time<Second>>> for Quantity<Length<Meter>> {
    type Output = Quantity<Velocity<MeterPerSecond>>;

    fn div(self, rhs: Quantity<Time<Second>>) -> Self::Output {
        Quantity { value: self.value / rhs.value, d: PhantomData::default(), }
    }
}

// units

pub struct Meter;
pub struct Second;

pub struct MeterPerSecond;

impl Unit for Meter {}
impl Unit for Second {}
impl Unit for MeterPerSecond {}
