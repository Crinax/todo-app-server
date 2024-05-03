use std::collections::HashSet;

use crate::core::entities::{rules::string_based_id::StringBasedId, task::Task};

#[derive(Clone, Debug)]
pub struct TaskWindow {
    tasks: HashSet<Task>,
}

impl TaskWindow {
    pub fn new() -> Self {
        Self {
            tasks: HashSet::new(),
        }
    }

    pub fn add(&mut self, task: Task) {
        self.tasks.insert(task);
    }

    pub fn tasks(&self) -> &HashSet<Task> {
        &self.tasks
    }

    pub fn remove(&mut self, task_id: &StringBasedId) -> Option<Task> {
        let task = self.tasks.iter().find(|task| task.id() == task_id);
        let task = task?.clone();

        self.tasks.take(&task)
    }

    pub fn has(&self, task_id: &StringBasedId) -> bool {
        self.tasks.iter().any(|task| task.id() == task_id)
    }
}
