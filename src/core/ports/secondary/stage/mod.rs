use crate::core::entities::{
    rules::{min_len_string::MinLenString, string_based_id::StringBasedId},
    stage::Stage,
    stage_window::StageWindow,
};

pub trait LoadManyStagesPort {
    type LoadManyStagesPortError;

    async fn load_many_stages(&self) -> Result<StageWindow, Self::LoadManyStagesPortError>;
}

pub trait LoadStagePort {
    type LoadStagePortError;

    async fn load_stage(&self, stage_id: &StringBasedId)
        -> Result<Stage, Self::LoadStagePortError>;
}

pub trait CreateStagePort {
    type CreateStagePortError;

    async fn create_stage(
        &self,
        name: &MinLenString<1>,
    ) -> Result<Stage, Self::CreateStagePortError>;
}

pub trait SaveStagePort {
    type SaveStagePortError;

    async fn save_stage(&self, stage: &Stage) -> Result<Stage, Self::SaveStagePortError>;
}
