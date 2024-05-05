use crate::core::{
    entities::stage::Stage,
    ports::primary::commands::stage::{
        create_stage::CreateStageCommand, update_stage_name::UpdateStageNameCommand,
    },
};

pub trait CreateStageUseCase {
    type CreateStageUseCaseError;

    async fn create_stage(
        &self,
        command: CreateStageCommand,
    ) -> Result<Stage, Self::CreateStageUseCaseError>;
}

pub trait UpdateStageNameUseCase {
    type UpdateStageNameUseCaseError;

    async fn update_stage_name(
        &self,
        command: UpdateStageNameCommand,
    ) -> Result<Stage, Self::UpdateStageNameUseCaseError>;
}
