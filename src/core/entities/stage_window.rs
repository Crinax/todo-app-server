use crate::core::entities::{rules::string_based_id::StringBasedId, stage::Stage};

pub struct StageWindow {
    stages: Vec<Stage>,
}

impl StageWindow {
    pub fn new() -> Self {
        Self { stages: vec![] }
    }

    pub fn add(&mut self, stage: Stage) {
        if !self.has(&stage) {
            self.stages.push(stage);
        }
    }

    pub fn stages(&self) -> &[Stage] {
        &self.stages
    }

    pub fn remove(&mut self, stage: &Stage) {
        self.stages.retain(|s| s != stage);
    }

    pub fn has(&self, stage: &Stage) -> bool {
        self.stages.contains(stage)
    }

    pub fn move_task(
        &mut self,
        task_id: &StringBasedId,
        from_stage_id: &StringBasedId,
        to_stage_id: &StringBasedId,
    ) {
        if let Some(stage) = self.stages.iter_mut().find(|s| s.id() == from_stage_id.0) {
            stage.
        }
    }
}
