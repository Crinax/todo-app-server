use crate::core::{
    common::rules::{Id, Order, Rule},
    task::{
        ports::{primary::queries::GetTaskQuery, secondary::loaders::LoadTaskPort},
        rules::TaskId,
    },
};

pub struct GetTaskService<T: LoadTaskPort> {
    port: T,
}

impl<T: LoadTaskPort> GetTaskService<T> {
    pub fn new(port: T) -> Self {
        Self { port }
    }
}

pub struct GetTaskStruct {
    pub id: String,
    pub order: i32,
    pub title: String,
}

pub enum GetTaskError<T> {
    ParseId,
    Load(T),
}

impl<T: LoadTaskPort> GetTaskQuery for GetTaskService<T> {
    type Err = GetTaskError<T::Err>;
    type Res = GetTaskStruct;

    async fn get_task(&self, id: String) -> Result<Self::Res, Self::Err> {
        let task_id = TaskId::apply(id).map_err(|_| GetTaskError::ParseId)?;

        let task = self.port.load(task_id).await.map_err(GetTaskError::Load)?;

        Ok(GetTaskStruct {
            id: task.id().to_string(),
            order: task.order(),
            title: task.title().to_string(),
        })
    }
}
