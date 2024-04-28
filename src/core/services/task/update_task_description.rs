use crate::core::{
    entities::task::Task,
    ports::{
        primary::task::{
            commands::update_task_description::UpdateTaskDescriptionCommand,
            use_cases::UpdateTaskDescriptionUseCase,
        },
        secondary::task::{LoadTaskPort, SaveTaskPort},
    },
    services::Either,
};

struct UpdateTaskDescriptionService<T: SaveTaskPort, R: LoadTaskPort> {
    save_port: T,
    load_port: R,
}

impl<T: SaveTaskPort, R: LoadTaskPort> UpdateTaskDescriptionService<T, R> {
    fn new(save_port: T, load_port: R) -> Self {
        Self {
            save_port,
            load_port,
        }
    }
}

impl<T: SaveTaskPort, R: LoadTaskPort> UpdateTaskDescriptionUseCase
    for UpdateTaskDescriptionService<T, R>
{
    type UpdateTaskDescriptionUseCaseError = Either<T::SaveTaskPortError, R::LoadTaskPortError>;

    async fn update_task_description(
        &self,
        command: UpdateTaskDescriptionCommand,
    ) -> Result<Task, Self::UpdateTaskDescriptionUseCaseError> {
        let mut task = self
            .load_port
            .load_task(command.id())
            .await
            .map_err(Either::Right)?;

        task.update_description(command.description().clone());

        self.save_port.save_task(&task).await.map_err(Either::Left)
    }
}
