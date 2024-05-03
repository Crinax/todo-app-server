use std::collections::HashSet;

use crate::core::entities::{task::Task, task_window::TaskWindow};

#[test]
fn task_window_can_add_task_with_different_id() {
    let task_1 = Task::new(
        "1".parse().unwrap(),
        "task 1".parse().unwrap(),
        "simple task".parse().unwrap(),
    );

    let task_2 = Task::new(
        "2".parse().unwrap(),
        "task 2".parse().unwrap(),
        "simple task".parse().unwrap(),
    );

    let mut task_window = TaskWindow::new();

    task_window.add(task_1.clone());
    task_window.add(task_2.clone());

    assert_eq!(task_window.tasks(), &HashSet::from([task_1, task_2]));
}

#[test]
fn task_window_cannot_add_task_with_same_id() {
    let task_1 = Task::new(
        "1".parse().unwrap(),
        "task 1".parse().unwrap(),
        "simple task".parse().unwrap(),
    );

    let mut task_window = TaskWindow::new();

    task_window.add(task_1.clone());
    task_window.add(task_1.clone());

    assert_eq!(task_window.tasks(), &HashSet::from([task_1]));
}
