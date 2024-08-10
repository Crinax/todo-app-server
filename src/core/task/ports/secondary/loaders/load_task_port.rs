use crate::core::task::{enitities::Task, rules::TaskId};

pub trait LoadTaskPort {
    type Err;

    async fn load(&self, id: TaskId) -> Result<Task, Self::Err>;
}
