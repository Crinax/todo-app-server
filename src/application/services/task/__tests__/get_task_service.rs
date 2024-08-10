use crate::{
    application::services::task::get_task_service::GetTaskService,
    core::{
        common::rules::Rule,
        task::{
            enitities::Task,
            ports::{primary::queries::GetTaskQuery, secondary::loaders::LoadTaskPort},
            rules::{TaskId, TaskOrder, TaskTitle},
        },
    },
};

struct LoadTaskAdapter;

impl LoadTaskPort for LoadTaskAdapter {
    type Err = ();

    async fn load(&self, id: TaskId) -> Result<Task, Self::Err> {
        Ok(Task::new(
            id,
            TaskOrder::apply(1).unwrap(),
            TaskTitle::apply("title".to_string()).unwrap(),
        ))
    }
}

#[tokio::test]
async fn can_get_task() {
    let service = GetTaskService::new(LoadTaskAdapter {});

    let task = service.get_task("1".to_owned()).await;

    assert!(task.is_ok());

    let task = task.unwrap();

    assert_eq!(task.id, "1");
    assert_eq!(task.title, "title");
}
