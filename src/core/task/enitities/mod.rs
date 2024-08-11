use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate::core::common::{
    rules::{Id, MutableOrder, Order, Rule},
    AsShared,
};

use super::rules::{TaskId, TaskOrder, TaskTitle};

pub struct Task {
    id: TaskId,
    order: TaskOrder,
    title: TaskTitle,
}

impl Task {
    pub fn new(id: TaskId, order: TaskOrder, title: TaskTitle) -> Self {
        Self { id, order, title }
    }

    pub fn title(&self) -> &str {
        self.title.inner_ref()
    }

    pub fn update_title(&mut self, title: TaskTitle) -> () {
        self.title = title;
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

impl AsShared for Task {
    fn as_shared(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }

    fn as_atomic_shared(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }
}
