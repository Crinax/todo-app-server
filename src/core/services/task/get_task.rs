use crate::core::{
    entities::{rules::string_based_id::StringBasedId, task::Task},
    ports::{
        primary::{task::GetTaskByIdQuery, Query},
        secondary::task::LoadTaskPort,
    },
};

pub struct GetTaskService<T: LoadTaskPort> {
    port: T,
    id: StringBasedId,
}

impl<T: LoadTaskPort> GetTaskService<T> {
    pub fn new(port: T, id: StringBasedId) -> Self {
        Self { port, id }
    }
}

impl<T: LoadTaskPort> Query<Option<Task>> for GetTaskService<T> {
    async fn execute(&self) -> Option<Task> {
        self.port.load_task(&self.id).await
    }
}

impl<T: LoadTaskPort> GetTaskByIdQuery for GetTaskService<T> {}
