use crate::core::entities::rules::min_len_string::MinLenString;

pub enum CreateStageCommandError {
    NameIsEmpty,
}

pub struct CreateStageCommand {
    name: MinLenString<1>,
}

impl CreateStageCommand {
    pub fn new(string_name: String) -> Result<Self, CreateStageCommandError> {
        Ok(Self {
            name: string_name
                .parse()
                .map_err(|_| CreateStageCommandError::NameIsEmpty)?,
        })
    }

    pub fn name(&self) -> &MinLenString<1> {
        &self.name
    }
}
