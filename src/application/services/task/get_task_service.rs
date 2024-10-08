use crate::core::{
    common::{
        rules::{Id, Order, Rule},
        AsShared,
    },
    task::{
        ports::{primary::queries::GetTaskQuery, secondary::loaders::LoadTaskPort},
        rules::TaskId,
    },
};

pub struct GetTaskService<T: LoadTaskPort> {
    adapter: T,
}

impl<T: LoadTaskPort> GetTaskService<T> {
    pub fn new(adapter: T) -> Self {
        Self { adapter }
    }
}

#[derive(Debug, Clone)]
pub struct GetTaskResponse {
    pub id: String,
    pub order: i32,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GetTaskError<T> {
    ParseId,
    Load(T),
}

impl<T: LoadTaskPort> GetTaskQuery for GetTaskService<T> {
    type Err = GetTaskError<T::Err>;
    type Res = GetTaskResponse;

    async fn get_task(&self, id: String) -> Result<Self::Res, Self::Err> {
        let task_id = TaskId::apply(id).map_err(|_| GetTaskError::ParseId)?;

        let task = self
            .adapter
            .load(task_id)
            .await
            .map_err(GetTaskError::Load)?;

        Ok(GetTaskResponse {
            id: task.id().to_owned(),
            order: task.order(),
            title: task.title().to_owned(),
        })
    }
}
