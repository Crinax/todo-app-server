use std::hash::Hash;

use crate::core::entities::{
    rules::{min_len_string::MinLenString, string_based_id::StringBasedId},
    task::Task,
    task_window::TaskWindow,
    window::Id,
};

#[derive(Debug, Clone)]
pub struct Stage {
    id: StringBasedId,
    name: MinLenString<1>,
    task_window: TaskWindow,
}

impl Hash for Stage {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Stage {
    pub fn new(id: StringBasedId, name: MinLenString<1>) -> Self {
        Self {
            id,
            name,
            task_window: TaskWindow::new(),
        }
    }

    pub fn task_window(&self) -> &TaskWindow {
        &self.task_window
    }

    pub fn name(&self) -> &MinLenString<1> {
        &self.name
    }

    pub fn add_task(&mut self, task: Task) {
        self.task_window.add(task);
    }

    pub fn has_task(&self, task: &StringBasedId) -> bool {
        self.task_window.has(task)
    }

    pub fn remove_task(&mut self, task_id: &StringBasedId) -> Option<Task> {
        self.task_window.remove(task_id)
    }

    pub fn update_name(&mut self, name: MinLenString<1>) {
        self.name = name;
    }
}

impl Id for Stage {
    fn id(&self) -> &StringBasedId {
        &self.id
    }
}

impl PartialEq for Stage {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Stage {}
