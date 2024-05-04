use crate::core::entities::{rules::string_based_id::StringBasedId, task::Task, window::Window};

#[derive(Clone, Debug)]
pub struct TaskWindow {
    window: Window<Task>,
}

impl TaskWindow {
    pub fn new() -> Self {
        Self {
            window: Window::new(),
        }
    }

    pub fn add(&mut self, task: Task) {
        self.window.add(task);
    }

    pub fn tasks(&self) -> &[Task] {
        self.window.collection()
    }

    pub fn remove(&mut self, task_id: &StringBasedId) -> Option<Task> {
        self.window.remove(task_id)
    }

    pub fn has(&self, task_id: &StringBasedId) -> bool {
        self.window.has(task_id)
    }
}
