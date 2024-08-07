use crate::{
    common::traits::AsInner,
    core_v2::{
        common::rules::EntityId,
        task::{
            entity::{rules::TaskTitle, Task},
            ports::{
                primary::{commands::UpdateTaskNameCommand, use_cases::UpdateTaskNameUseCase},
                secondary::{LoadTaskPort, SaveTaskPort},
            },
            services::UpdateTaskNameService,
        },
    },
};

#[derive(Debug)]
struct LoadTaskAdapter {}

impl LoadTaskPort for LoadTaskAdapter {
    type Err = ();

    async fn load_task(&self, _id: &EntityId) -> Result<Task, Self::Err> {
        Ok(Task::new(
            "1".parse().unwrap(),
            "task 1".parse().unwrap(),
            "Veryvery".parse().unwrap(),
            "2".parse().unwrap(),
        ))
    }
}

#[derive(Debug)]
struct SaveTaskAdapter {}

impl SaveTaskPort for SaveTaskAdapter {
    type Err = ();

    async fn save_task(&self, task: Task) -> Result<Task, Self::Err> {
        Ok(task)
    }
}

#[tokio::test]
async fn task_update_name() {
    let task_id: EntityId = "1".parse().unwrap();
    let load_adapter = LoadTaskAdapter {};
    let save_adapter = SaveTaskAdapter {};

    let service = UpdateTaskNameService::new(save_adapter, load_adapter);

    let new_name: TaskTitle = "task 2".parse().unwrap();
    let command = UpdateTaskNameCommand::new(task_id.clone(), new_name.clone());

    let new_task = service.update_name(command).await;

    assert!(new_task.is_ok());

    let new_task = new_task.unwrap();

    assert_eq!(new_task.title().as_inner(), new_name.as_inner());
}
