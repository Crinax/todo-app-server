use crate::core::entities::rules::min_len_string::MinLenString;

use super::rules::string_based_id::StringBasedId;

pub struct Task {
    id: StringBasedId,
    name: MinLenString<1>,
    desctription: MinLenString<0>,
}
