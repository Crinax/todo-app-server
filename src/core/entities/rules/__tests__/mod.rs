use crate::core::entities::rules::{
    min_len_string::{MinLenString, MinLenStringError},
    string_based_id::{StringBasedId, StringBasedIdParseError},
    BusinessRule,
};

#[test]
fn creating_string_based_id_with_no_empty_string_is_ok() {
    let id = StringBasedId::parse("1234567890".to_owned());

    assert!(id.is_ok());
}

#[test]
fn creating_string_based_id_with_empty_string_is_err() {
    let id = StringBasedId::parse("".to_owned());

    assert!(id.is_err());
    assert_eq!(id.err().unwrap(), StringBasedIdParseError::EmptyId);
}

#[test]
fn creating_min_len_string_with_eq_count_of_chars_is_ok() {
    let something = MinLenString::<3>::parse("123".to_owned());

    assert!(something.is_ok());
}

#[test]
fn creating_min_len_string_with_non_eq_count_of_chars_is_err() {
    let something = MinLenString::<3>::parse("12".to_owned());

    assert!(something.is_err());
    assert_eq!(something.err().unwrap(), MinLenStringError::TooShort);
}
