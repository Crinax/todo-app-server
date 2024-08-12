use crate::core::common::rules::{Id, Rule};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskId(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskIdRuleErr {
    Empty,
}

impl Rule<String> for TaskId {
    type Err = TaskIdRuleErr;

    fn apply(input: String) -> Result<Self, Self::Err> {
        if input.is_empty() {
            return Err(TaskIdRuleErr::Empty);
        }

        Ok(TaskId(input))
    }

    fn inner(self) -> String {
        self.0
    }

    fn inner_ref(&self) -> &String {
        &self.0
    }
}

impl Id for TaskId {
    fn id(&self) -> &str {
        &self.0
    }
}
