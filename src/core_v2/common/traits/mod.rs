use crate::core_v2::common::rules::StringBasedId;

pub trait Id {
    fn id(&self) -> &StringBasedId;
}

pub trait Order {
    fn order(&self) -> Order;
}
