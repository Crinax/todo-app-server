use std::sync::Arc;

use crate::core::{
    entities::{
        rules::{min_len_string::MinLenString, string_based_id::StringBasedId, BuisnessRule},
        task::Task,
    },
    ports::{
        primary::task::{
            commands::update_task_description::UpdateTaskDescriptionCommand,
            use_cases::UpdateTaskDescriptionUseCase,
        },
        secondary::task::{LoadTaskPort, SaveTaskPort},
    },
    services::task::update_task_description::UpdateTaskDescriptionService,
};

#[derive(Clone)]
struct LoadSaveTaskAdapter {}

impl SaveTaskPort for LoadSaveTaskAdapter {
    type SaveTaskPortError = ();

    async fn save_task(&self, task: &Task) -> Result<Task, Self::SaveTaskPortError> {
        Ok(task.clone())
    }
}

impl LoadTaskPort for LoadSaveTaskAdapter {
    type LoadTaskPortError = ();

    async fn load_task(&self, task_id: &StringBasedId) -> Result<Task, Self::LoadTaskPortError> {
        let name = MinLenString::<1>::parse("task name".to_owned()).unwrap();
        let description = MinLenString::<0>::parse("".to_owned()).unwrap();
        let id = task_id.clone();

        Ok(Task::new(id, name, description))
    }
}

#[tokio::test]
async fn update_task_description() {
    let adapter = LoadSaveTaskAdapter {};
    let service = UpdateTaskDescriptionService::new(adapter.clone(), adapter);
    let command = UpdateTaskDescriptionCommand::new("1".to_owned(), "1234".to_owned()).unwrap();

    let result = service.update_task_description(command).await;

    assert!(result.is_ok());

    let task = result.unwrap();

    assert_eq!(task.description(), "1234");
}
