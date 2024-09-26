use crate::core::common::rules::{Id, Rule};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnId(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskIdRuleErr {
    Empty,
}

impl Rule<String> for ColumnId {
    type Err = TaskIdRuleErr;

    fn apply(input: String) -> Result<Self, Self::Err> {
        if input.is_empty() {
            return Err(TaskIdRuleErr::Empty);
        }

        Ok(ColumnId(input))
    }

    fn inner(self) -> String {
        self.0
    }

    fn inner_ref(&self) -> &String {
        &self.0
    }
}

impl Id for ColumnId {
    fn id(&self) -> &str {
        &self.0
    }
}
