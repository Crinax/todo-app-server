pub mod fixed_string;
pub mod id;
pub mod order;

pub use fixed_string::MinLenString;
pub use id::EntityId;
pub use order::EntityOrder;

pub trait BusinessRule {
    type Error;

    fn parse(input: String) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn value(&self) -> &str;
}
