use crate::core_v2::{common::rules::EntityId, task::entity::Task};

pub trait LoadTaskPort {
    type Err;

    async fn load_task(&self, id: EntityId) -> Result<Task, Self::Err>;
}

pub trait LoadTasksByColumn {
    type Err;

    async fn load_task_by_column(&self, column_id: EntityId) -> Result<Vec<Task>, Self::Err>;
}

pub trait SaveTaskPort {
    type Err;

    async fn save_task(&self, task: Task) -> Result<Task, Self::Err>;
}
