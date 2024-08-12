use super::mock::load_task_adapter::SuccessLoadTaskAdapter;
use crate::{
    application::services::task::{
        __tests__::mock::load_task_adapter::{LoadTaskError, MaybeNotFoundLoadTaskAdapter},
        get_task_service::{GetTaskError, GetTaskService},
    },
    core::{
        common::rules::Rule,
        task::{
            enitities::Task,
            ports::primary::queries::GetTaskQuery,
            rules::{TaskId, TaskOrder, TaskTitle},
        },
    },
};

#[tokio::test]
async fn can_get_task_successfully() {
    let service = GetTaskService::new(SuccessLoadTaskAdapter {});

    let task = service.get_task("1".to_owned()).await;

    assert!(task.is_ok());

    let task = task.unwrap();

    assert_eq!(task.id, "1");
    assert_eq!(task.title, "title");
}

#[tokio::test]
async fn find_task_with_error() {
    let service = GetTaskService::new(MaybeNotFoundLoadTaskAdapter(vec![
        Task::new(
            TaskId::apply("1".to_string()).unwrap(),
            TaskOrder::apply(1).unwrap(),
            TaskTitle::apply("title 1".to_string()).unwrap(),
        ),
        Task::new(
            TaskId::apply("2".to_string()).unwrap(),
            TaskOrder::apply(2).unwrap(),
            TaskTitle::apply("title 2".to_string()).unwrap(),
        ),
    ]));

    let task_with_id_1 = service.get_task("1".to_owned()).await;
    let task_with_id_3 = service.get_task("3".to_owned()).await;

    assert!(task_with_id_1.is_ok());
    assert!(task_with_id_3.is_err());

    let task_with_id_1 = task_with_id_1.unwrap();
    let task_with_id_3 = task_with_id_3.unwrap_err();

    assert_eq!(task_with_id_1.id, "1");
    assert_eq!(task_with_id_1.title, "title 1");
    assert_eq!(task_with_id_3, GetTaskError::Load(LoadTaskError::NotFound));
}
