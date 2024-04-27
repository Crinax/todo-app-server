use crate::core::entities::{
    rules::{min_len_string::MinLenString, string_based_id::StringBasedId},
    task::Task,
};

pub trait LoadManyTasksPort {
    async fn load_many_tasks(&self) -> Vec<Task>;
}

pub trait LoadTaskPort {
    async fn load_task(&self, task_id: &StringBasedId) -> Option<Task>;
}

pub trait SearchTasksByNamePort {
    async fn search_tasks_by_name(&self, name: &MinLenString<3>) -> Vec<Task>;
}

pub trait CreateTaskPort {
    type CreateTaskPortError;

    async fn save_task(
        &self,
        name: &MinLenString<3>,
        description: &MinLenString<3>,
    ) -> Result<Task, Self::CreateTaskPortError>;
}
