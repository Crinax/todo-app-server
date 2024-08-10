use crate::core::task::enitities::Task;

pub trait UpdateTaskPort {
    type Err;

    async fn update_task(&self, task: &Task) -> Result<(), Self::Err>;
}
