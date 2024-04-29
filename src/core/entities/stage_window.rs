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

    pub fn remove(&mut self, stage_id: &StringBasedId) -> Option<Stage> {
        let index = self.stages.iter().position(|stage| stage.id() == stage_id);

        if let Some(index) = index {
            return Some(self.stages.remove(index));
        }

        None
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
        let from_stage = self.stages.iter_mut().find(|s| s.id() == from_stage_id);
        let mut task = None;

        if let Some(stage) = from_stage {
            task = stage.task_window_mut().remove(task_id)
        }

        if task.is_none() {
            return;
        }

        let to_stage = self.stages.iter_mut().find(|s| s.id() == to_stage_id);

        if let Some(stage) = to_stage {
            stage.task_window_mut().add(task.unwrap())
        }
    }
}
