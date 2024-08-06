use crate::core_v2::common::rules::MinLenString;

pub type TaskTitle = MinLenString<1>;
pub type TaskDescription = MinLenString<0>;
