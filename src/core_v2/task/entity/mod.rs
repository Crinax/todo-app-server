use crate::core_v2::common::rules::{EntityId, MinLenString};

pub struct Task {
    id: EntityId,
    title: MinLenString<1>,
    description: MinLenString<0>,
    column_id: EntityId,
}
