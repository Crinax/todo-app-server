use crate::core::{
    common::rules::{Id, Rule},
    task::{
        enitities::Task,
        ports::secondary::loaders::LoadTaskPort,
        rules::{TaskId, TaskOrder, TaskTitle},
    },
};

pub struct SuccessLoadTaskAdapter;

impl LoadTaskPort for SuccessLoadTaskAdapter {
    type Err = ();

    async fn load(&self, id: TaskId) -> Result<Task, Self::Err> {
        Ok(Task::new(
            id,
            TaskOrder::apply(1).unwrap(),
            TaskTitle::apply("title".to_string()).unwrap(),
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadTaskError {
    NotFound,
}

pub struct MaybeNotFoundLoadTaskAdapter(pub Vec<Task>);

impl LoadTaskPort for MaybeNotFoundLoadTaskAdapter {
    type Err = LoadTaskError;

    async fn load(&self, id: TaskId) -> Result<Task, Self::Err> {
        self.0
            .iter()
            .find(|task| task.id() == id.id())
            .cloned()
            .ok_or(LoadTaskError::NotFound)
    }
}
