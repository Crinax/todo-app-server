use crate::core::{
    entities::stage::Stage,
    ports::{
        primary::{
            commands::stage::update_stage_name::UpdateStageNameCommand,
            use_cases::stage::UpdateStageNameUseCase,
        },
        secondary::stage::{LoadStagePort, SaveStagePort},
    },
    services::Either,
};

struct UpdateStageNameService<S: SaveStagePort, G: LoadStagePort> {
    save_port: S,
    load_port: G,
}

impl<S: SaveStagePort, G: LoadStagePort> UpdateStageNameService<S, G> {
    fn new(save_port: S, load_port: G) -> Self {
        Self {
            save_port,
            load_port,
        }
    }
}

impl<S: SaveStagePort, G: LoadStagePort> UpdateStageNameUseCase for UpdateStageNameService<S, G> {
    type UpdateStageNameUseCaseError = Either<S::SaveStagePortError, G::LoadStagePortError>;

    async fn update_stage_name(
        &self,
        command: UpdateStageNameCommand,
    ) -> Result<Stage, Self::UpdateStageNameUseCaseError> {
        let mut stage = self
            .load_port
            .load_stage(command.id())
            .await
            .map_err(Either::Right)?;

        stage.update_name(command.name().clone());

        self.save_port
            .save_stage(&stage)
            .await
            .map_err(Either::Left)
    }
}
