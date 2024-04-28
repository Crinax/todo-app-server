use crate::core::entities::rules::BusinessRule;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MinLenStringError {
    TooShort,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MinLenString<const N: usize>(pub String);

impl<const N: usize> BusinessRule for MinLenString<N> {
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
