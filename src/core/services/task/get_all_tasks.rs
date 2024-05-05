use crate::core::{
    entities::{task::Task, window::Window},
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
    type GetAllTasksQueryError = T::LoadManyTasksPortError;

    async fn get_all_tasks(&self) -> Result<Window<Task>, Self::GetAllTasksQueryError> {
        self.port.load_many_tasks().await
    }
}
