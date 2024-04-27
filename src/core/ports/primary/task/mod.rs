use crate::core::entities::task::Task;

pub trait GetAllTasksQuery {
    async fn execute<T>(&self) -> Vec<T>
    where
        T: Into<Task>;
}
