use super::BuisnessRule;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringBasedIdParseError {
    EmptyId,
}

pub struct StringBasedId(pub String);

impl BuisnessRule for StringBasedId {
    type Error = StringBasedIdParseError;

    fn parse(input: String) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        if input.is_empty() {
            Err(StringBasedIdParseError::EmptyId)
        } else {
            Ok(StringBasedId(input))
        }
    }
}
