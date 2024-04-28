use crate::core::entities::rules::{min_len_string::MinLenString, BuisnessRule};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CreateTaskCommandError {
    NameTooShort,
}

pub struct CreateTaskCommand {
    name: MinLenString<1>,
    description: MinLenString<0>,
}

impl CreateTaskCommand {
    pub fn new(
        string_name: String,
        string_description: String,
    ) -> Result<Self, CreateTaskCommandError> {
        let name =
            MinLenString::parse(string_name).map_err(|_| CreateTaskCommandError::NameTooShort)?;

        // NOTE: never return an error here
        let description = MinLenString::parse(string_description).unwrap();

        Ok(Self { name, description })
    }

    pub fn name(&self) -> &MinLenString<1> {
        &self.name
    }

    pub fn description(&self) -> &MinLenString<0> {
        &self.description
    }
}
