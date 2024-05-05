use crate::core::entities::{
    rules::{min_len_string::MinLenString, string_based_id::StringBasedId},
    task::Task,
    window::Window,
};

pub trait LoadManyTasksPort {
    type LoadManyTasksPortError;

    async fn load_many_tasks(&self) -> Result<Window<Task>, Self::LoadManyTasksPortError>;
}

pub trait LoadTaskPort {
    type LoadTaskPortError;

    async fn load_task(&self, task_id: &StringBasedId) -> Result<Task, Self::LoadTaskPortError>;
}

pub trait SearchTasksByNamePort {
    type SearchTasksByNamePortError;

    async fn search_tasks_by_name(
        &self,
        name: &MinLenString<1>,
    ) -> Result<Window<Task>, Self::SearchTasksByNamePortError>;
}

pub trait CreateTaskPort {
    type CreateTaskPortError;

    async fn create_task(
        &self,
        name: &MinLenString<1>,
        description: &MinLenString<0>,
    ) -> Result<Task, Self::CreateTaskPortError>;
}

pub trait SaveTaskPort {
    type SaveTaskPortError;

    async fn save_task(&self, task: &Task) -> Result<Task, Self::SaveTaskPortError>;
}
