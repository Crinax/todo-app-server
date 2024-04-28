use crate::core::entities::{
    rules::{string_based_id::StringBasedId, BusinessRule},
    task::Task,
};

#[derive(Clone, Debug)]
pub struct TaskWindow {
    tasks: Vec<Task>,
}

impl TaskWindow {
    pub fn new() -> Self {
        Self { tasks: vec![] }
    }

    pub fn add(&mut self, task: Task) {
        // TODO: think about how to simplify it
        // maybe return an StringBasedId ref instead of &str ref
        // or implement PartialEq<&str> and PartalEq<String> for StringBasedId
        let id = StringBasedId::parse(task.id().to_string()).unwrap();

        if !self.has(&id) {
            self.tasks.push(task);
        }
    }

    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }

    pub fn remove(&mut self, task: &Task) {
        self.tasks.retain(|t| t != task);
    }

    pub fn has(&self, task_id: &StringBasedId) -> bool {
        self.tasks
            .iter()
            .find(|task| task.id() == task_id.0)
            .is_some()
    }
}
