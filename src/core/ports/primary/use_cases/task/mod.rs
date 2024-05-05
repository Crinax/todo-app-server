use crate::core::{
    entities::task::Task,
    ports::primary::commands::task::{
        create_task::CreateTaskCommand, update_task_description::UpdateTaskDescriptionCommand,
        update_task_name::UpdateTaskNameCommand,
    },
};

pub trait CreateTaskUseCase {
    type CreateTaskUseCaseError;

    async fn create_task(
        &self,
        command: CreateTaskCommand,
    ) -> Result<Task, Self::CreateTaskUseCaseError>;
}

pub trait UpdateTaskNameUseCase {
    type UpdateTaskNameUseCaseError;

    async fn update_task_name(
        &self,
        command: UpdateTaskNameCommand,
    ) -> Result<Task, Self::UpdateTaskNameUseCaseError>;
}

pub trait UpdateTaskDescriptionUseCase {
    type UpdateTaskDescriptionUseCaseError;

    async fn update_task_description(
        &self,
        command: UpdateTaskDescriptionCommand,
    ) -> Result<Task, Self::UpdateTaskDescriptionUseCaseError>;
}
