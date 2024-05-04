use crate::core::entities::{rules::string_based_id::StringBasedId, stage::Stage, window::Window};

pub struct StageWindow {
    window: Window<Stage>,
}

impl StageWindow {
    pub fn new() -> Self {
        Self {
            window: Window::new(),
        }
    }

    pub fn add(&mut self, stage: Stage) {
        self.window.add(stage);
    }

    pub fn stages(&self) -> &[Stage] {
        self.window.collection()
    }

    pub fn remove(&mut self, stage_id: &StringBasedId) -> Option<Stage> {
        self.window.remove(stage_id)
    }

    pub fn has(&self, stage_id: &StringBasedId) -> bool {
        self.window.has(stage_id)
    }

    pub fn move_task(
        &mut self,
        task_id: &StringBasedId,
        from_stage_id: &StringBasedId,
        to_stage_id: &StringBasedId,
    ) {
        let from_stage = self.remove(from_stage_id);

        if from_stage.is_none() {
            return;
        }

        let mut from_stage = from_stage.unwrap();
        let to_stage = self.remove(to_stage_id);

        if to_stage.is_none() {
            self.add(from_stage);
            return;
        }

        let mut to_stage = to_stage.unwrap();

        let task = from_stage.remove_task(task_id);

        if task.is_none() {
            self.add(to_stage);
            self.add(from_stage);
            return;
        }

        let task = task.unwrap();

        to_stage.add_task(task);

        self.add(from_stage);
        self.add(to_stage);
    }
}
