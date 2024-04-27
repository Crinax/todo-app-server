use crate::core::entities::{rules::string_based_id::StringBasedId, task::Task};

pub trait LoadManyTasksPort {
    async fn load_many_tasks(&self) -> Vec<Task>;
}

pub trait LoadTaskPort {
    async fn load_task(&self, task_id: &StringBasedId) -> Option<Task>;
}
