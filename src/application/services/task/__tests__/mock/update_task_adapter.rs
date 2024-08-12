use crate::core::task::{enitities::Task, ports::secondary::UpdateTaskPort};

pub struct SuccessUpdateTaskAdapter;

impl UpdateTaskPort for SuccessUpdateTaskAdapter {
    type Err = ();

    async fn update_task(&self, _task: &Task) -> Result<(), Self::Err> {
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateTaskError {
    NotFound,
}

pub struct NotFoundUpdateTaskAdapter;

impl UpdateTaskPort for NotFoundUpdateTaskAdapter {
    type Err = UpdateTaskError;

    async fn update_task(&self, _task: &Task) -> Result<(), Self::Err> {
        Err(UpdateTaskError::NotFound)
    }
}
