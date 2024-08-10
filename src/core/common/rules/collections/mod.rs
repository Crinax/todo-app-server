use super::{MutableOrder, Order};

pub trait SortableEntityCollection {
    fn sort_entities(&mut self) -> ();
}

pub trait ReordableEntityCollection: SortableEntityCollection {
    type Err;

    fn reorder_entities(&mut self) -> Vec<Result<(), Self::Err>>;
}

impl<T: Order> SortableEntityCollection for Vec<T> {
    fn sort_entities(&mut self) {
        self.sort_by_key(|entity| entity.order());
    }
}

impl<T: MutableOrder> ReordableEntityCollection for Vec<T> {
    type Err = <T as MutableOrder>::Err;

    fn reorder_entities(&mut self) -> Vec<Result<(), Self::Err>> {
        let mut result: Vec<Result<(), Self::Err>> = vec![];

        for (index, entity) in self.iter_mut().enumerate() {
            result.push(entity.set_order(index as i32));
        }

        result
    }
}
