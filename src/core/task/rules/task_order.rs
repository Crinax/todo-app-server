use crate::core::common::rules::{MutableOrder, Order, Rule};

pub struct TaskOrder(i32);

impl Rule<i32> for TaskOrder {
    type Err = ();

    fn apply(input: i32) -> Result<Self, Self::Err> {
        Ok(TaskOrder(input))
    }

    fn inner(self) -> i32 {
        self.0
    }

    fn inner_ref(&self) -> &i32 {
        &self.0
    }
}

impl Order for TaskOrder {
    fn order(&self) -> i32 {
        *self.inner_ref()
    }
}

impl MutableOrder for TaskOrder {
    type Err = <TaskOrder as Rule<i32>>::Err;

    fn set_order(&mut self, order: i32) -> Result<(), Self::Err> {
        self.0 = TaskOrder::apply(order)?.inner();

        Ok(())
    }
}
