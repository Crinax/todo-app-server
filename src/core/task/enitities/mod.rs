use crate::core::common::rules::{Id, MutableOrder, Order, Rule};

use super::rules::{TaskId, TaskOrder, TaskTitle};

pub struct Task {
    id: TaskId,
    order: TaskOrder,
    title: TaskTitle,
}

impl Task {
    pub fn title(&self) -> &str {
        self.title.inner_ref()
    }
}

impl Id for Task {
    fn id(&self) -> &str {
        self.id.id()
    }
}

impl Order for Task {
    fn order(&self) -> i32 {
        self.order.order()
    }
}

impl MutableOrder for Task {
    type Err = <TaskOrder as MutableOrder>::Err;

    fn set_order(&mut self, order: i32) -> Result<(), Self::Err> {
        self.order.set_order(order)
    }
}
