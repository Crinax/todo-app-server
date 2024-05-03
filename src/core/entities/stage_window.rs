use std::collections::HashSet;

use crate::core::entities::{rules::string_based_id::StringBasedId, stage::Stage};

pub struct StageWindow {
    stages: HashSet<Stage>,
}

impl StageWindow {
    pub fn new() -> Self {
        Self {
            stages: HashSet::new(),
        }
    }

    pub fn add(&mut self, stage: Stage) {
        if !self.has(&stage) {
            self.stages.insert(stage);
        }
    }

    pub fn stages(&self) -> &HashSet<Stage> {
        &self.stages
    }

    pub fn remove(&mut self, stage_id: &StringBasedId) -> Option<Stage> {
        let stage = self.stages.iter().find(|stage| stage.id() == stage_id);
        let stage = stage?.clone();

        self.stages.take(&stage)
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
        let from_stage = self.remove(from_stage_id);
        let to_stage = self.remove(to_stage_id);

        if from_stage.is_none() {
            return;
        }

        if to_stage.is_none() {
            return;
        }

        let mut from_stage = from_stage.unwrap();
        let mut to_stage = to_stage.unwrap();

        let task = from_stage.remove_task(task_id);

        if task.is_none() {
            return;
        }

        let task = task.unwrap();

        to_stage.add_task(task);
    }
}
