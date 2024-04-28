use crate::core::{
    entities::task::Task,
    ports::{
        primary::task::{
            commands::update_task_name::UpdateTaskNameCommand, use_cases::UpdateTaskNameUseCase,
        },
        secondary::task::{LoadTaskPort, SaveTaskPort},
    },
    services::Either,
};

struct UpdateTaskNameService<T: SaveTaskPort, R: LoadTaskPort> {
    save_port: T,
    load_port: R,
}

impl<T: SaveTaskPort, R: LoadTaskPort> UpdateTaskNameService<T, R> {
    fn new(save_port: T, load_port: R) -> Self {
        Self {
            save_port,
            load_port,
        }
    }
}

impl<T: SaveTaskPort, R: LoadTaskPort> UpdateTaskNameUseCase for UpdateTaskNameService<T, R> {
    type UpdateTaskNameUseCaseError = Either<T::SaveTaskPortError, R::LoadTaskPortError>;

    async fn update_task_name(
        &self,
        command: UpdateTaskNameCommand,
    ) -> Result<Task, Self::UpdateTaskNameUseCaseError> {
        let mut task = self
            .load_port
            .load_task(command.id())
            .await
            .map_err(Either::Right)?;

        task.update_name(command.name().clone());

        self.save_port.save_task(&task).await.map_err(Either::Left)
    }
}
