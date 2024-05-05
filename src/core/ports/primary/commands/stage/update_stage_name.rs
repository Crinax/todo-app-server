use crate::core::entities::rules::{min_len_string::MinLenString, string_based_id::StringBasedId};

pub enum UpdateStageNameCommandError {
    IdIsEmpty,
    NameIsEmpty,
}

pub struct UpdateStageNameCommand {
    id: StringBasedId,
    name: MinLenString<1>,
}

impl UpdateStageNameCommand {
    pub fn new(
        string_id: String,
        string_name: String,
    ) -> Result<Self, UpdateStageNameCommandError> {
        Ok(Self {
            id: string_id
                .parse()
                .map_err(|_| UpdateStageNameCommandError::IdIsEmpty)?,
            name: string_name
                .parse()
                .map_err(|_| UpdateStageNameCommandError::NameIsEmpty)?,
        })
    }

    pub fn id(&self) -> &StringBasedId {
        &self.id
    }

    pub fn name(&self) -> &MinLenString<1> {
        &self.name
    }
}
