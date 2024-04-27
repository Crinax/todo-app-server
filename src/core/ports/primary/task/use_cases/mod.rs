use crate::core::{
    entities::task::Task, ports::primary::task::commands::create_task::CreateTaskCommand,
};

trait CreateTaskUseCase {
    type CreateTaskUseCaseError;

    async fn create_task(
        &self,
        command: CreateTaskCommand,
    ) -> Result<Task, Self::CreateTaskUseCaseError>;
}
