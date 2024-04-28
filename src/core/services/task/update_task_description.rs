use crate::core::{
    entities::task::Task,
    ports::{
        primary::task::{
            commands::update_task_description::UpdateTaskDescriptionCommand,
            use_cases::UpdateTaskDescriptionUseCase,
        },
        secondary::task::UpdateTaskDescriptionPort,
    },
};

struct UpdateTaskDescriptionService<T: UpdateTaskDescriptionPort> {
    port: T,
}

impl<T: UpdateTaskDescriptionPort> UpdateTaskDescriptionService<T> {
    fn new(port: T) -> Self {
        Self { port }
    }
}

impl<T: UpdateTaskDescriptionPort> UpdateTaskDescriptionUseCase
    for UpdateTaskDescriptionService<T>
{
    type UpdateTaskDescriptionUseCaseError = T::UpdateTaskDescriptionPortError;

    async fn update_task_description(
        &self,
        command: UpdateTaskDescriptionCommand,
    ) -> Result<Task, Self::UpdateTaskDescriptionUseCaseError> {
        self.port
            .update_task_name(command.id(), command.description())
            .await
    }
}
