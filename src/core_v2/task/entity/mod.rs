use crate::core_v2::common::rules::MinLenString;

pub struct Task {
    id: MinLenString<1>,
    title: MinLenString<1>,
    description: MinLenString<0>,
}
