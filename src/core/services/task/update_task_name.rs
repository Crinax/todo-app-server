use crate::core::{
    entities::task::Task,
    ports::{
        primary::task::{
            commands::update_task_name::UpdateTaskNameCommand, use_cases::UpdateTaskNameUseCase,
        },
        secondary::task::UpdateTaskNamePort,
    },
};

struct UpdateTaskNameService<T: UpdateTaskNamePort> {
    port: T,
}

impl<T: UpdateTaskNamePort> UpdateTaskNameService<T> {
    fn new(port: T) -> Self {
        Self { port }
    }
}

impl<T: UpdateTaskNamePort> UpdateTaskNameUseCase for UpdateTaskNameService<T> {
    type UpdateTaskNameUseCaseError = T::UpdateTaskNamePortError;

    async fn update_task_name(
        &self,
        command: UpdateTaskNameCommand,
    ) -> Result<Task, Self::UpdateTaskNameUseCaseError> {
        self.port
            .update_task_name(command.id(), command.name())
            .await
    }
}
