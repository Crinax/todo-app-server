use crate::core::entities::rules::BuisnessRule;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NonEmptyStringError {
    Empty,
}

pub struct MinLenString<const N: usize>(pub String);

impl<const N: usize> BuisnessRule for MinLenString<N> {
    type Error = NonEmptyStringError;

    fn parse(input: String) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        if input.len() >= N {
            Err(NonEmptyStringError::Empty)
        } else {
            Ok(MinLenString(input))
        }
    }
}
