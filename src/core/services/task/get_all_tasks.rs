use crate::core::{
    entities::task::Task,
    ports::{
        primary::{task::GetAllTasksQuery, Query},
        secondary::task::LoadManyTasksPort,
    },
};

pub struct GetAllTasksService<T: LoadManyTasksPort> {
    port: T,
}

impl<T: LoadManyTasksPort> GetAllTasksService<T> {
    pub fn new(port: T) -> Self {
        Self { port }
    }
}

impl<T: LoadManyTasksPort> Query<Vec<Task>> for GetAllTasksService<T> {
    async fn execute(&self) -> Vec<Task> {
        self.port.load_many_tasks().await
    }
}

impl<T: LoadManyTasksPort> GetAllTasksQuery for GetAllTasksService<T> {}
