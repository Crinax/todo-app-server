pub mod fixed_string;
pub mod id;
pub mod order;

pub use fixed_string::MinLenString;
pub use id::Id;
pub use order::Order;

pub trait BusinessRule {
    type Error;

    fn parse(input: String) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn value(&self) -> &str;
}
