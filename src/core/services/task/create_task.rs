use crate::core::{
    entities::task::Task,
    ports::{
        primary::task::{commands::create_task::CreateTaskCommand, use_cases::CreateTaskUseCase},
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
            .create_task(command.get_name(), command.get_description())
            .await
    }
}
