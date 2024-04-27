use crate::core::{
    entities::task::Task,
    ports::{primary::task::queries::GetAllTasksQuery, secondary::task::LoadManyTasksPort},
};

pub struct GetAllTasksService<T: LoadManyTasksPort> {
    port: T,
}

impl<T: LoadManyTasksPort> GetAllTasksService<T> {
    pub fn new(port: T) -> Self {
        Self { port }
    }
}

impl<T: LoadManyTasksPort> GetAllTasksQuery for GetAllTasksService<T> {
    async fn get_all_tasks(&self) -> Vec<Task> {
        self.port.load_many_tasks().await
    }
}
