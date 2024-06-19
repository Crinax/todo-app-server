use crate::core_v2::common::rules::{EntityId, EntityOrder};

pub trait Id {
    fn id(&self) -> &EntityId;
}

pub trait Order {
    fn order(&self) -> EntityOrder;
}
