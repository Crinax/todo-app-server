use crate::core_v2::{
    common::rules::EntityId,
    task::{
        entity::Task,
        ports::{primary::queries::GetTaskQuery, secondary::ports::LoadTaskPort},
    },
};

pub struct GetTaskService<T: LoadTaskPort> {
    port: T,
}

impl<T: LoadTaskPort> GetTaskQuery for GetTaskService<T> {
    type Err = T::Err;

    async fn get_task(&self, id: EntityId) -> Result<Task, Self::Err> {
        self.port.load_task(id).await
    }
}