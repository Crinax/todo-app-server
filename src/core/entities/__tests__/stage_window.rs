use std::collections::HashSet;

use crate::core::entities::{stage::Stage, stage_window::StageWindow};

#[test]
fn stage_window_can_add_stage() {
    let stage_1 = Stage::new("1".parse().unwrap(), "stage 1".parse().unwrap());
    let stage_2 = Stage::new("2".parse().unwrap(), "stage 2".parse().unwrap());
    let mut stage_window = StageWindow::new();

    stage_window.add(stage_1.clone());
    stage_window.add(stage_2.clone());

    assert_eq!(stage_window.stages(), &HashSet::from([stage_1, stage_2]));
}

#[test]
fn stage_window_can_remove_stage() {
    let stage_1 = Stage::new("1".parse().unwrap(), "stage 1".parse().unwrap());
    let stage_2 = Stage::new("2".parse().unwrap(), "stage 2".parse().unwrap());
    let mut stage_window = StageWindow::new();

    stage_window.add(stage_1.clone());
    stage_window.add(stage_2.clone());

    assert_eq!(stage_window.remove(stage_2.id()), Some(stage_2));
    assert_eq!(stage_window.stages(), &HashSet::from([stage_1]));
}
