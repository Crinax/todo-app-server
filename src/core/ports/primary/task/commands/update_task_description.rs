use crate::core::entities::rules::{
    min_len_string::MinLenString, string_based_id::StringBasedId, BuisnessRule,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UpdateTaskDescriptionCommandError {
    IdIsEmpty,
}

pub struct UpdateTaskDescriptionCommand {
    id: StringBasedId,
    description: MinLenString<0>,
}

impl UpdateTaskDescriptionCommand {
    pub fn new(
        id: String,
        name: String,
        description: String,
    ) -> Result<Self, UpdateTaskDescriptionCommandError> {
        let id =
            StringBasedId::parse(id).map_err(|_| UpdateTaskDescriptionCommandError::IdIsEmpty)?;
        // NOTE: never return an error here
        let description = MinLenString::parse(name).unwrap();

        Ok(Self { id, description })
    }

    pub fn description(&self) -> &MinLenString<0> {
        &self.description
    }

    pub fn id(&self) -> &StringBasedId {
        &self.id
    }
}
