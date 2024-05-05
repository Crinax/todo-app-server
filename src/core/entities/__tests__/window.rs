use crate::core::entities::{
    rules::string_based_id::StringBasedId,
    window::{Id, Window},
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Item {
    id: StringBasedId,
    name: String,
    order: i32,
}

impl Item {
    fn new(id: StringBasedId, name: String, order: i32) -> Self {
        Self { id, name, order }
    }

    fn new_uncheck(id: &str, name: &str, order: i32) -> Self {
        Self::new(id.to_owned().parse().unwrap(), name.to_owned(), order)
    }

    fn order(&self) -> i32 {
        self.order
    }
}

impl Id for Item {
    fn id(&self) -> &StringBasedId {
        &self.id
    }
}

#[test]
fn can_add_an_item_to_window() {
    let mut window: Window<Item> = Window::new();

    window.add(Item::new_uncheck("1", "1", 0));

    assert_eq!(window.collection(), &[Item::new_uncheck("1", "1", 0)]);
}

#[test]
fn can_remove_an_item_from_window() {
    let mut window: Window<Item> = Window::new();

    window.add(Item::new_uncheck("1", "1", 0));

    window.remove(&"1".parse().unwrap());

    assert_eq!(window.collection(), &[]);
}

#[test]
fn cannot_add_item_with_same_id() {
    let mut window: Window<Item> = Window::new();

    window.add(Item::new_uncheck("1", "1", 0));
    window.add(Item::new_uncheck("1", "1", 0));

    assert_eq!(window.collection(), &[Item::new_uncheck("1", "1", 0)]);
}

#[test]
fn cannot_remove_non_existing_item() {
    let mut window: Window<Item> = Window::new();

    window.add(Item::new_uncheck("1", "1", 0));
    window.remove(&"2".parse().unwrap());

    assert_eq!(window.collection(), &[Item::new_uncheck("1", "1", 0)]);
}

#[test]
fn can_check_if_item_is_in_window() {
    let mut window: Window<Item> = Window::new();

    window.add(Item::new_uncheck("1", "1", 0));
    window.add(Item::new_uncheck("2", "2", 1));

    assert!(window.has(&"1".parse().unwrap()));
    assert!(!window.has(&"3".parse().unwrap()));
}

#[test]
fn can_get_item_by_id() {
    let mut window: Window<Item> = Window::new();

    window.add(Item::new_uncheck("1", "1", 0));
    window.add(Item::new_uncheck("2", "2", 1));

    assert_eq!(
        window.get(&"1".parse().unwrap()),
        Some(&Item::new_uncheck("1", "1", 0))
    );
    assert_eq!(window.get(&"3".parse().unwrap()), None);
}

#[test]
fn can_mutate_item_by_id() {
    let mut window: Window<Item> = Window::new();

    window.add(Item::new_uncheck("1", "1", 0));

    let item = window.get_mut(&"1".parse().unwrap()).unwrap();

    item.name = "2".to_owned();
    item.order = 1;

    assert_eq!(window.collection(), &[Item::new_uncheck("1", "2", 1)]);
}

#[test]
fn can_sort_by_key() {
    let mut window: Window<Item> = Window::new();

    window.add(Item::new_uncheck("1", "1", 2));
    window.add(Item::new_uncheck("2", "2", 1));
    window.add(Item::new_uncheck("3", "3", 0));

    window.sort_by_key(|item| item.order());

    assert_eq!(
        window.collection(),
        &[
            Item::new_uncheck("3", "3", 0),
            Item::new_uncheck("2", "2", 1),
            Item::new_uncheck("1", "1", 2)
        ]
    );

    window.sort_by_key(|item| -item.order());

    assert_eq!(
        window.collection(),
        &[
            Item::new_uncheck("1", "1", 2),
            Item::new_uncheck("2", "2", 1),
            Item::new_uncheck("3", "3", 0)
        ]
    );
}
