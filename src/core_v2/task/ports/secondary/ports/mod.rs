use crate::core_v2::{common::rules::EntityId, task::entity::Task};

pub trait LoadTaskPort {
    type Err;

    async fn load_task(&self, id: EntityId) -> Result<Task, Self::Err>;
}
