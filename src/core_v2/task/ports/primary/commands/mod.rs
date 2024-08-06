use crate::core_v2::common::rules::{EntityId, MinLenString};

pub struct UpdateTaskNameCommand {
    id: EntityId,
    name: MinLenString<1>,
}

impl UpdateTaskNameCommand {
    fn new(id: EntityId, name: MinLenString<1>) -> Self {
        Self { id, name }
    }

    fn id(&self) -> &EntityId {
        &self.id
    }

    fn name(&self) -> &MinLenString<1> {
        &self.name
    }
}
