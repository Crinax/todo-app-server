use crate::core::entities::rules::BuisnessRule;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MinLenStringError {
    TooShort,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MinLenString<const N: usize>(pub String);

impl<const N: usize> BuisnessRule for MinLenString<N> {
    type Error = MinLenStringError;

    fn parse(input: String) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        if input.len() < N {
            Err(MinLenStringError::TooShort)
        } else {
            Ok(MinLenString(input))
        }
    }
}
