use crate::core::{
    entities::{rules::min_len_string::MinLenString, task::Task, window::Window},
    ports::{
        primary::task::queries::SearchTasksByNameQuery, secondary::task::SearchTasksByNamePort,
    },
};

pub struct SearchTasksByNameService<T: SearchTasksByNamePort> {
    port: T,
}

impl<T: SearchTasksByNamePort> SearchTasksByNameService<T> {
    pub fn new(port: T) -> Self {
        Self { port }
    }
}

impl<T: SearchTasksByNamePort> SearchTasksByNameQuery for SearchTasksByNameService<T> {
    type SearchTasksByNameQueryError = T::SearchTasksByNamePortError;

    async fn search_tasks_by_name(
        &self,
        name: &MinLenString<1>,
    ) -> Result<Window<Task>, Self::SearchTasksByNameQueryError> {
        self.port.search_tasks_by_name(name).await
    }
}
