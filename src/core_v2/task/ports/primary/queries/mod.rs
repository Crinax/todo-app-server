use crate::core_v2::{common::rules::EntityId, task::entity::Task};

pub trait GetTaskQuery {
    type Err;

    async fn get_task(id: EntityId) -> Result<Task, Self::Err>;
}

pub trait GetAllTaskInColumnQuery {
    type Err;

    async fn get_task_by_column(column_id: EntityId) -> Result<Vec<Task>, Self::Err>;
}
