pub mod min_len_string;
pub mod string_based_id;

#[cfg(test)]
mod __tests__;

// TODO: add an value method for getting inner value of the rule
pub trait BusinessRule {
    type Error;

    fn parse(input: String) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
