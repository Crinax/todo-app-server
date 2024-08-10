use crate::core::common::rules::{Id, Rule};

pub struct TaskId(String);

impl Rule<String> for TaskId {
    type Err = ();

    fn apply(input: String) -> Result<Self, Self::Err> {
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
