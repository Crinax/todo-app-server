pub mod min_len_string;
pub mod string_based_id;

#[cfg(test)]
mod __tests__;

pub trait BuisnessRule {
    type Error;

    fn parse(input: String) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
