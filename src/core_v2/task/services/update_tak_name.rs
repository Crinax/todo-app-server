use crate::core_v2::task::{
    entity::Task,
    ports::{
        primary::{commands::UpdateTaskNameCommand, use_cases::UpdateTaskNameUseCase},
        secondary::{LoadTaskPort, SaveTaskPort},
    },
};

pub struct UpdateTaskNameService<S: SaveTaskPort, L: LoadTaskPort> {
    save_port: S,
    load_port: L,
}

impl<S: SaveTaskPort, L: LoadTaskPort> UpdateTaskNameService<S, L> {
    pub fn new(save_port: S, load_port: L) -> Self {
        Self {
            save_port,
            load_port,
        }
    }
}

#[derive(Debug)]
pub enum UpdateTaskNameUseCaseErr<S: SaveTaskPort, L: LoadTaskPort> {
    SaveErr(S::Err),
    LoadErr(L::Err),
}

impl<S: SaveTaskPort, L: LoadTaskPort> UpdateTaskNameUseCase for UpdateTaskNameService<S, L> {
    type Err = UpdateTaskNameUseCaseErr<S, L>;

    async fn update_name(&self, command: UpdateTaskNameCommand) -> Result<Task, Self::Err> {
        let task = self
            .load_port
            .load_task(command.id())
            .await
            .map_err(|e| UpdateTaskNameUseCaseErr::LoadErr(e))?;

        let task = task.update(Some(command.name().clone()), None, None);

        self.save_port
            .save_task(task)
            .await
            .map_err(|e| UpdateTaskNameUseCaseErr::SaveErr(e))
    }
}
