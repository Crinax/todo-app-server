use crate::core::common::rules::Rule;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskTitle(String);

#[derive(Debug)]
pub enum TaskTitleRuleErr {
    Empty,
}

impl Rule<String> for TaskTitle {
    type Err = TaskTitleRuleErr;

    fn apply(input: String) -> Result<Self, Self::Err> {
        if input.is_empty() {
            Err(TaskTitleRuleErr::Empty)
        } else {
            Ok(TaskTitle(input))
        }
    }

    fn inner(self) -> String {
        self.0
    }

    fn inner_ref(&self) -> &String {
        &self.0
    }
}
