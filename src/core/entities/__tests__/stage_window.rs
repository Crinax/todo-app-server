use crate::core::entities::{stage::Stage, stage_window::StageWindow, task::Task, window::Id};

#[test]
fn stage_window_can_add_stage() {
    let stage_1 = Stage::new("1".parse().unwrap(), "stage 1".parse().unwrap());
    let stage_2 = Stage::new("2".parse().unwrap(), "stage 2".parse().unwrap());
    let mut stage_window = StageWindow::new();

    stage_window.add(stage_1.clone());
    stage_window.add(stage_2.clone());

    assert_eq!(stage_window.stages(), &[stage_1, stage_2]);
}

#[test]
fn stage_window_can_remove_stage() {
    let stage_1 = Stage::new("1".parse().unwrap(), "stage 1".parse().unwrap());
    let stage_2 = Stage::new("2".parse().unwrap(), "stage 2".parse().unwrap());
    let mut stage_window = StageWindow::new();

    stage_window.add(stage_1.clone());
    stage_window.add(stage_2.clone());

    assert_eq!(stage_window.remove(stage_2.id()), Some(stage_2));
    assert_eq!(stage_window.stages(), &[stage_1]);
}

#[test]
fn stage_window_cannot_add_stage_with_same_id() {
    let stage = Stage::new("1".parse().unwrap(), "stage 1".parse().unwrap());
    let mut stage_window = StageWindow::new();

    stage_window.add(stage.clone());
    stage_window.add(stage.clone());

    assert_eq!(stage_window.stages(), &[stage]);
}

#[test]
fn stage_window_cannot_remove_not_existing_stage() {
    let stage = Stage::new("1".parse().unwrap(), "stage 1".parse().unwrap());
    let mut stage_window = StageWindow::new();

    assert_eq!(stage_window.remove(stage.id()), None);
    assert_eq!(stage_window.stages(), &[]);
}

#[test]
fn stage_window_has_returns_true_for_existing_stage() {
    let stage = Stage::new("1".parse().unwrap(), "stage 1".parse().unwrap());
    let mut stage_window = StageWindow::new();

    stage_window.add(stage.clone());

    assert!(stage_window.has(stage.id()));
}

#[test]
fn stage_window_has_returns_false_for_not_existing_stage() {
    let stage = Stage::new("1".parse().unwrap(), "stage 1".parse().unwrap());
    let stage_window = StageWindow::new();

    assert!(!stage_window.has(stage.id()));
}

#[test]
fn stage_window_move_task_from_one_stage_to_another() {
    let task = Task::new(
        "1".parse().unwrap(),
        "task 1".parse().unwrap(),
        "1".parse().unwrap(),
    );
    let mut stage_1 = Stage::new("1".parse().unwrap(), "stage 1".parse().unwrap());
    let stage_2 = Stage::new("2".parse().unwrap(), "stage 2".parse().unwrap());
    let mut stage_window = StageWindow::new();

    let stage_1_id = stage_1.id().clone();
    let stage_2_id = stage_2.id().clone();

    stage_1.add_task(task.clone());

    stage_window.add(stage_1);
    stage_window.add(stage_2);

    stage_window.move_task(task.id(), &stage_1_id, &stage_2_id);

    let stage_1 = stage_window.remove(&stage_1_id).unwrap();
    let stage_2 = stage_window.remove(&stage_2_id).unwrap();

    assert_eq!(stage_1.task_window().tasks(), &[]);
    assert_eq!(stage_2.task_window().tasks(), &[task]);
}
