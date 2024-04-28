use crate::core::{
    entities::{rules::string_based_id::StringBasedId, task::Task},
    ports::{primary::task::queries::GetTaskByIdQuery, secondary::task::LoadTaskPort},
};

pub struct GetTaskService<T: LoadTaskPort> {
    port: T,
}

impl<T: LoadTaskPort> GetTaskService<T> {
    pub fn new(port: T) -> Self {
        Self { port }
    }
}

impl<T: LoadTaskPort> GetTaskByIdQuery for GetTaskService<T> {
    type GetTaskByIdQueryError = T::LoadTaskPortError;

    async fn get_task_by_id(
        &self,
        id: &StringBasedId,
    ) -> Result<Task, Self::GetTaskByIdQueryError> {
        self.port.load_task(id).await
    }
}
