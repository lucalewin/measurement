use std::marker::PhantomData;

fn main() {}

trait DimType {}
impl DimType for isize {}

struct Dimension<L, M, T>
where 
    L: DimType,
    M: DimType,
    T: DimType
{
    l: PhantomData<L>,
    m: PhantomData<M>,
    t: PhantomData<T>
}

trait DimensionTrait {}
impl<L, M, T> DimensionTrait for Dimension<L, M, T>
where 
    L: DimType,
    M: DimType,
    T: DimType {}

struct Quantity<D> {
    d: PhantomData<D>,
    value: f64
}

fn test() {

}