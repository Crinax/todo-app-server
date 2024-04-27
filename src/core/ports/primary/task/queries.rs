use crate::core::entities::{
    rules::{min_len_string::MinLenString, string_based_id::StringBasedId},
    task::Task,
};

pub trait GetAllTasksQuery {
    async fn get_all_tasks(&self) -> Vec<Task>;
}

pub trait GetTaskByIdQuery {
    async fn get_task_by_id(&self, id: &StringBasedId) -> Option<Task>;
}

pub trait SearchTasksByNameQuery {
    async fn search_tasks_by_name(&self, name: &MinLenString<3>) -> Vec<Task>;
}
