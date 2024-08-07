use crate::core_v2::{common::rules::EntityId, task::entity::Task};

pub trait GetTaskQuery {
    type Err;

    async fn get_task(&self, id: &EntityId) -> Result<Task, Self::Err>;
}

pub trait GetTasksInColumnQuery {
    type Err;

    async fn get_tasks_by_column(&self, column_id: &EntityId) -> Result<Vec<Task>, Self::Err>;
}
