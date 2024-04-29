use crate::core::entities::{rules::string_based_id::StringBasedId, task::Task};

#[derive(Clone, Debug)]
pub struct TaskWindow {
    tasks: Vec<Task>,
}

impl TaskWindow {
    pub fn new() -> Self {
        Self { tasks: vec![] }
    }

    pub fn add(&mut self, task: Task) {
        if !self.has(&task.id()) {
            self.tasks.push(task);
        }
    }

    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }

    pub fn remove(&mut self, task_id: &StringBasedId) -> Option<Task> {
        let index = self.tasks.iter().position(|task| task.id() == task_id);

        if let Some(index) = index {
            return Some(self.tasks.remove(index));
        }

        None
    }

    pub fn has(&self, task_id: &StringBasedId) -> bool {
        self.tasks
            .iter()
            .find(|task| task.id() == task_id)
            .is_some()
    }
}
