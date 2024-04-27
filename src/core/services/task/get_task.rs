use crate::core::{
    entities::{rules::string_based_id::StringBasedId, task::Task},
    ports::{primary::task::GetTaskByIdQuery, secondary::task::LoadTaskPort},
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
    async fn get_task_by_id(&self, id: &StringBasedId) -> Option<Task> {
        self.port.load_task(id).await
    }
}
