pub mod min_len_string;
pub mod string_based_id;

pub trait BuisnessRule {
    type Error;

    fn parse(input: String) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
