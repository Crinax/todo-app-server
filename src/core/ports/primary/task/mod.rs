use crate::core::{entities::task::Task, ports::primary::Query};

pub trait GetAllTasksQuery: Query<Vec<Task>> {}
pub trait GetTaskByIdQuery: Query<Option<Task>> {}
pub trait SearchTasksByName: Query<Vec<Task>> {}
