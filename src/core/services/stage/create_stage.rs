use crate::core::{
    entities::stage::Stage,
    ports::{
        primary::{
            commands::stage::create_stage::CreateStageCommand, use_cases::stage::CreateStageUseCase,
        },
        secondary::stage::CreateStagePort,
    },
};

struct CreateStageService<T: CreateStagePort> {
    port: T,
}

impl<T: CreateStagePort> CreateStageService<T> {
    pub fn new(port: T) -> Self {
        Self { port }
    }
}

impl<T: CreateStagePort> CreateStageUseCase for CreateStageService<T> {
    type CreateStageUseCaseError = T::CreateStagePortError;
    async fn create_stage(
        &self,
        command: CreateStageCommand,
    ) -> Result<Stage, Self::CreateStageUseCaseError> {
        self.port.create_stage(command.name()).await
    }
}
