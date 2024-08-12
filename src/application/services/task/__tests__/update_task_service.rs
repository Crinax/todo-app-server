use crate::{
    application::services::task::{
        __tests__::mock::{
            load_task_adapter::{
                LoadTaskError, MaybeNotFoundLoadTaskAdapter, SuccessLoadTaskAdapter,
            },
            update_task_adapter::{
                NotFoundUpdateTaskAdapter, SuccessUpdateTaskAdapter, UpdateTaskError,
            },
        },
        update_task_service::{UpdateTaskService, UpdateTaskTitleError},
    },
    core::task::ports::primary::{
        commands::UpdateTaskTitleCommand, use_cases::UpdateTaskTitleUseCase,
    },
};

#[tokio::test]
async fn can_update_task_successfully() {
    let service = UpdateTaskService::new(SuccessLoadTaskAdapter {}, SuccessUpdateTaskAdapter {});

    let task = service
        .update_title(UpdateTaskTitleCommand::new(
            "1".to_owned(),
            "another title".to_owned(),
        ))
        .await;

    assert!(task.is_ok());

    let task = task.unwrap();

    assert_eq!(task.title, "another title");
}

#[tokio::test]
async fn can_update_task_with_not_found_save_error() {
    let service = UpdateTaskService::new(SuccessLoadTaskAdapter {}, NotFoundUpdateTaskAdapter {});

    let task = service
        .update_title(UpdateTaskTitleCommand::new(
            "1".to_owned(),
            "another title".to_owned(),
        ))
        .await;

    assert!(task.is_err());

    let task = task.unwrap_err();

    assert_eq!(task, UpdateTaskTitleError::Save(UpdateTaskError::NotFound));
}

#[tokio::test]
async fn can_update_task_with_not_found_load_error() {
    let service = UpdateTaskService::new(
        MaybeNotFoundLoadTaskAdapter(vec![]),
        SuccessUpdateTaskAdapter {},
    );

    let task = service
        .update_title(UpdateTaskTitleCommand::new(
            "1".to_owned(),
            "another title".to_owned(),
        ))
        .await;

    assert!(task.is_err());

    let task = task.unwrap_err();

    assert_eq!(task, UpdateTaskTitleError::Load(LoadTaskError::NotFound));
}
