use crate::core::entities::{
    rules::{min_len_string::MinLenString, string_based_id::StringBasedId},
    task::Task,
    window::Window,
};

pub trait GetAllTasksQuery {
    type GetAllTasksQueryError;

    async fn get_all_tasks(&self) -> Result<Window<Task>, Self::GetAllTasksQueryError>;
}

pub trait GetTaskByIdQuery {
    type GetTaskByIdQueryError;

    async fn get_task_by_id(&self, id: &StringBasedId)
        -> Result<Task, Self::GetTaskByIdQueryError>;
}

pub trait SearchTasksByNameQuery {
    type SearchTasksByNameQueryError;

    async fn search_tasks_by_name(
        &self,
        name: &MinLenString<1>,
    ) -> Result<Window<Task>, Self::SearchTasksByNameQueryError>;
}
