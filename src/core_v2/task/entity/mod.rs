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
    pub fn new(
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

    pub fn id(&self) -> &EntityId {
        &self.id
    }

    pub fn title(&self) -> &TaskTitle {
        &self.title
    }

    pub fn description(&self) -> &TaskDescription {
        &self.description
    }

    pub fn column_id(&self) -> &EntityId {
        &self.column_id
    }

    pub fn update(
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
