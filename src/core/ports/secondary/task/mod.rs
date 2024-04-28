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
    async fn search_tasks_by_name(&self, name: &MinLenString<1>) -> Vec<Task>;
}

pub trait CreateTaskPort {
    type CreateTaskPortError;

    async fn create_task(
        &self,
        name: &MinLenString<1>,
        description: &MinLenString<0>,
    ) -> Result<Task, Self::CreateTaskPortError>;
}

pub trait UpdateTaskNamePort {
    type UpdateTaskNamePortError;

    async fn update_task_name(
        &self,
        id: &StringBasedId,
        name: &MinLenString<1>,
    ) -> Result<Task, Self::UpdateTaskNamePortError>;
}

pub trait UpdateTaskDescriptionPort {
    type UpdateTaskDescriptionPortError;

    async fn update_task_name(
        &self,
        id: &StringBasedId,
        description: &MinLenString<0>,
    ) -> Result<Task, Self::UpdateTaskDescriptionPortError>;
}
