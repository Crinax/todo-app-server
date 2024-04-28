use crate::core::entities::rules::{
    min_len_string::MinLenString, string_based_id::StringBasedId, BuisnessRule,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UpdateTaskNameCommandError {
    IdIsEmpty,
    NameTooShort,
}

pub struct UpdateTaskNameCommand {
    id: StringBasedId,
    name: MinLenString<1>,
}

impl UpdateTaskNameCommand {
    pub fn new(
        id: String,
        name: String,
        description: String,
    ) -> Result<Self, UpdateTaskNameCommandError> {
        let id = StringBasedId::parse(id).map_err(|_| UpdateTaskNameCommandError::IdIsEmpty)?;
        let name =
            MinLenString::parse(name).map_err(|_| UpdateTaskNameCommandError::NameTooShort)?;

        Ok(Self { id, name })
    }

    pub fn id(&self) -> &StringBasedId {
        &self.id
    }

    pub fn name(&self) -> &MinLenString<1> {
        &self.name
    }
}
