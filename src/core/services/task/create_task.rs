use crate::core::{
    entities::task::Task,
    ports::{
        primary::{
            commands::task::create_task::CreateTaskCommand, use_cases::task::CreateTaskUseCase,
        },
        secondary::task::CreateTaskPort,
    },
};

pub struct CreateTaskService<T: CreateTaskPort> {
    port: T,
}

impl<T: CreateTaskPort> CreateTaskService<T> {
    pub fn new(port: T) -> Self {
        Self { port }
    }
}

impl<T: CreateTaskPort> CreateTaskUseCase for CreateTaskService<T> {
    type CreateTaskUseCaseError = T::CreateTaskPortError;

    async fn create_task(
        &self,
        command: CreateTaskCommand,
    ) -> Result<Task, Self::CreateTaskUseCaseError> {
        self.port
            .create_task(command.name(), command.description())
            .await
    }
}
