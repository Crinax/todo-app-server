use crate::core_v2::task::ports::{
    primary::queries::GetTasksInColumnQuery, secondary::ports::LoadTasksByColumn,
};

pub struct GetTaskByColumn<T: LoadTasksByColumn> {
    port: T,
}

impl<T: LoadTasksByColumn> GetTaskByColumn<T> {
    pub fn new(port: T) -> Self {
        Self { port }
    }
}

impl<T: LoadTasksByColumn> GetTasksInColumnQuery for GetTaskByColumn<T> {
    type Err = T::Err;

    async fn get_task_by_column(
        &self,
        column_id: crate::core_v2::common::rules::EntityId,
    ) -> Result<Vec<crate::core_v2::task::entity::Task>, Self::Err> {
        self.port.load_task_by_column(column_id).await
    }
}
