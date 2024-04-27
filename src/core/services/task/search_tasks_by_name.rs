use crate::core::{
    entities::{rules::min_len_string::MinLenString, task::Task},
    ports::secondary::task::SearchTasksByNamePort,
};

pub struct SearchTasksByNameService<T: SearchTasksByNamePort> {
    port: T,
}

impl<T: SearchTasksByNamePort> SearchTasksByNameService<T> {
    pub fn new(port: T) -> Self {
        Self { port }
    }
}

impl<T: SearchTasksByNamePort> SearchTasksByNamePort for SearchTasksByNameService<T> {
    async fn search_tasks_by_name(&self, name: &MinLenString<3>) -> Vec<Task> {
        self.port.search_tasks_by_name(name).await
    }
}
