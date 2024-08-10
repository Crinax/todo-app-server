pub mod collections;

pub trait Rule<T>
where
    Self: Sized,
{
    type Err;

    fn apply(input: T) -> Result<Self, Self::Err>;

    fn inner(self) -> T;

    fn inner_ref(&self) -> &T;
}

pub trait Id {
    fn id(&self) -> &str;
}

pub trait Order {
    fn order(&self) -> i32;
}

pub trait MutableOrder: Order {
    type Err;

    fn set_order(&mut self, order: i32) -> Result<(), Self::Err>;
}
