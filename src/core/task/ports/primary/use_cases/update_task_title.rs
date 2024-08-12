use crate::core::task::ports::primary::commands::UpdateTaskTitleCommand;

pub trait UpdateTaskTitleUseCase {
    type Res;
    type Err;

    async fn update_title(&self, command: UpdateTaskTitleCommand) -> Result<Self::Res, Self::Err>;
}
