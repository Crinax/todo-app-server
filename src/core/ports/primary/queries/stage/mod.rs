use crate::core::entities::{rules::string_based_id::StringBasedId, stage_window::StageWindow};

pub trait GetAllStagesQuery {
    type GetAllStagesQueryError;

    fn get_all_stages(&self) -> Result<StageWindow, Self::GetAllStagesQueryError>;
}

pub trait GetStageQuery {
    type GetStageQueryError;

    fn get_stage(&self, id: &StringBasedId) -> Result<StageWindow, Self::GetStageQueryError>;
}
