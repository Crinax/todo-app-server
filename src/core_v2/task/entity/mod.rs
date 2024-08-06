pub mod rules;

use crate::core_v2::{
    common::rules::EntityId,
    task::entity::rules::{TaskDescription, TaskTitle},
};

#[derive(Clone, Debug)]
pub struct Task {
    id: EntityId,
    title: TaskTitle,
    description: TaskDescription,
    column_id: EntityId,
}

impl Task {
    fn new(
        id: EntityId,
        title: TaskTitle,
        description: TaskDescription,
        column_id: EntityId,
    ) -> Self {
        Self {
            id,
            title,
            description,
            column_id,
        }
    }

    fn id(&self) -> &EntityId {
        &self.id
    }

    fn title(&self) -> &TaskTitle {
        &self.title
    }

    fn description(&self) -> &TaskDescription {
        &self.description
    }

    fn column_id(&self) -> &EntityId {
        &self.column_id
    }

    fn update(
        self,
        title: Option<TaskTitle>,
        description: Option<TaskDescription>,
        column_id: Option<EntityId>,
    ) -> Self {
        Self {
            id: self.id,
            title: title.unwrap_or(self.title),
            description: description.unwrap_or(self.description),
            column_id: column_id.unwrap_or(self.column_id),
        }
    }
}
