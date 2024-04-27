use crate::core::entities::rules::min_len_string::MinLenString;

use super::rules::string_based_id::StringBasedId;

#[derive(Clone, Debug)]
pub struct Task {
    id: StringBasedId,
    name: MinLenString<1>,
    desctription: MinLenString<0>,
}

impl Task {
    pub fn new(id: StringBasedId, name: MinLenString<1>, desctription: MinLenString<0>) -> Self {
        Self {
            id,
            name,
            desctription,
        }
    }

    pub fn id(&self) -> &str {
        &self.id.0
    }

    pub fn name(&self) -> &str {
        &self.name.0
    }

    pub fn description(&self) -> &str {
        &self.desctription.0
    }
}
