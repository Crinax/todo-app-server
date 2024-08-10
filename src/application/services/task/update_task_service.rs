use crate::core::{
    common::rules::Rule,
    task::{
        enitities::Task,
        ports::{
            primary::{commands::UpdateTaskTitleCommand, use_cases::UpdateTaskTitleUseCase},
            secondary::{loaders::LoadTaskPort, UpdateTaskPort},
        },
        rules::{TaskId, TaskTitle},
    },
};

pub struct UpdateTaskService<S: UpdateTaskPort, L: LoadTaskPort> {
    save_adapter: S,
    load_adapter: L,
}

impl<S: UpdateTaskPort, L: LoadTaskPort> UpdateTaskService<S, L> {
    pub fn new(load_adapter: L, save_adapter: S) -> Self {
        Self {
            save_adapter,
            load_adapter,
        }
    }
}

pub enum UpdateTaskTitleError<LoadErr, SaveErr> {
    ParseId,
    ParseTitle,
    Load(LoadErr),
    Save(SaveErr),
}

impl<S: UpdateTaskPort, L: LoadTaskPort> UpdateTaskTitleUseCase for UpdateTaskService<S, L> {
    type Err = UpdateTaskTitleError<L::Err, S::Err>;

    async fn update_title(&self, command: UpdateTaskTitleCommand) -> Result<Task, Self::Err> {
        let task_id = TaskId::apply(command.id().to_owned()).map_err(|_| Self::Err::ParseId)?;
        let task_title =
            TaskTitle::apply(command.title().to_owned()).map_err(|_| Self::Err::ParseTitle)?;

        let mut task = self
            .load_adapter
            .load(task_id)
            .await
            .map_err(|err| Self::Err::Load(err))?;

        task.update_title(task_title);

        self.save_adapter
            .update_task(&task)
            .await
            .map_err(|err| Self::Err::Save(err))?;

        Ok(task)
    }
}
