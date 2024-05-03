use std::hash::Hash;

use crate::core::entities::rules::min_len_string::MinLenString;

use super::rules::string_based_id::StringBasedId;

#[derive(Clone, Debug)]
pub struct Task {
    id: StringBasedId,
    name: MinLenString<1>,
    description: MinLenString<0>,
}

impl Hash for Task {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Task {
    pub fn new(id: StringBasedId, name: MinLenString<1>, description: MinLenString<0>) -> Self {
        Self {
            id,
            name,
            description,
        }
    }

    pub fn id(&self) -> &StringBasedId {
        &self.id
    }

    pub fn name(&self) -> &MinLenString<1> {
        &self.name
    }

    pub fn description(&self) -> &MinLenString<0> {
        &self.description
    }

    pub fn update_name(&mut self, name: MinLenString<1>) {
        self.name = name
    }

    pub fn update_description(&mut self, description: MinLenString<0>) {
        self.description = description
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Task {}
