use crate::core::{
    entities::{
        rules::{min_len_string::MinLenString, string_based_id::StringBasedId, BusinessRule},
        task::Task,
        window::{Id, Window},
    },
    ports::{primary::queries::task::GetAllTasksQuery, secondary::task::LoadManyTasksPort},
    services::task::get_all_tasks::GetAllTasksService,
};

struct GetAllTasksAdapter {}

impl LoadManyTasksPort for GetAllTasksAdapter {
    type LoadManyTasksPortError = ();

    async fn load_many_tasks(&self) -> Result<Window<Task>, Self::LoadManyTasksPortError> {
        let name = MinLenString::<1>::parse("task name".to_owned()).unwrap();
        let description = MinLenString::<0>::parse("".to_owned()).unwrap();
        let id = StringBasedId::parse("1".to_owned()).unwrap();

        Ok(Window::from([Task::new(id, name, description)]))
    }
}

#[tokio::test]
async fn should_get_all_tasks() {
    let adapter = GetAllTasksAdapter {};
    let service = GetAllTasksService::new(adapter);

    let tasks = service.get_all_tasks().await.unwrap();

    assert!(!tasks.is_empty());

    let task = tasks.collection().first();

    assert!(task.is_some());

    let task = task.unwrap();

    assert_eq!(task.id(), &"1".parse().unwrap());
    assert_eq!(task.name(), &"task name".parse().unwrap());
    assert_eq!(task.description(), &"".parse().unwrap());
}
