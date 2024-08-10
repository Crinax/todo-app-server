use crate::core::task::{enitities::Task, ports::primary::commands::UpdateTaskTitleCommand};

pub trait UpdateTaskTitleUseCase {
    type Err;

    async fn update_title(&self, command: UpdateTaskTitleCommand) -> Result<Task, Self::Err>;
}
