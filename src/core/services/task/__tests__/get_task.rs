use crate::core::{
    entities::{
        rules::{min_len_string::MinLenString, string_based_id::StringBasedId, BusinessRule},
        task::Task,
        window::Id,
    },
    ports::{primary::queries::task::GetTaskByIdQuery, secondary::task::LoadTaskPort},
    services::task::get_task::GetTaskService,
};

struct LoadTaskByIdAdapter {}

impl LoadTaskPort for LoadTaskByIdAdapter {
    type LoadTaskPortError = ();

    async fn load_task(&self, task_id: &StringBasedId) -> Result<Task, Self::LoadTaskPortError> {
        let name = MinLenString::<1>::parse("task name".to_owned()).unwrap();
        let description = MinLenString::<0>::parse("".to_owned()).unwrap();
        let id = task_id.clone();

        Ok(Task::new(id, name, description))
    }
}

#[tokio::test]
async fn should_get_task() {
    let adapter = LoadTaskByIdAdapter {};
    let service = GetTaskService::new(adapter);

    let task = service
        .get_task_by_id(&StringBasedId::parse("1".to_owned()).unwrap())
        .await;

    assert!(task.is_ok());

    let task = task.unwrap();

    assert_eq!(task.id(), &"1".parse().unwrap());
    assert_eq!(task.name(), &"task name".parse().unwrap());
    assert_eq!(task.description(), &"".parse().unwrap());
}
