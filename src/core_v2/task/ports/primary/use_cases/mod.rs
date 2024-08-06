use crate::core_v2::task::entity::Task;

use super::commands::UpdateTaskNameCommand;

pub trait UpdateTaskNameUseCase {
    type Err;

    async fn update_name(&self, command: UpdateTaskNameCommand) -> Result<Task, Self::Err>;
}
