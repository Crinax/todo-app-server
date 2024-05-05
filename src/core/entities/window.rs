use std::cmp::Ordering;

use crate::core::entities::rules::string_based_id::StringBasedId;

pub trait Id {
    fn id(&self) -> &StringBasedId;
}

pub trait Order<T: Ord> {
    fn order(&self) -> T;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Window<T: Id> {
    collection: Vec<T>,
}

impl<T: Id> From<Vec<T>> for Window<T> {
    fn from(collection: Vec<T>) -> Self {
        Self { collection }
    }
}

impl<T: Id + Clone, const N: usize> From<[T; N]> for Window<T> {
    fn from(collection: [T; N]) -> Self {
        Self {
            collection: collection.to_vec(),
        }
    }
}

impl<T: Id> FromIterator<T> for Window<T> {
    fn from_iter<E: IntoIterator<Item = T>>(iter: E) -> Self {
        Self {
            collection: iter.into_iter().collect(),
        }
    }
}

impl<T: Id> Window<T> {
    pub fn new() -> Self {
        Self { collection: vec![] }
    }

    pub fn has(&self, id: &StringBasedId) -> bool {
        self.collection.iter().any(|item| item.id() == id)
    }

    pub fn collection(&self) -> &[T] {
        &self.collection
    }

    pub fn add(&mut self, item: T) {
        if !self.has(item.id()) {
            self.collection.push(item);
        }
    }

    pub fn remove(&mut self, id: &StringBasedId) -> Option<T> {
        let index = self.collection.iter().position(|item| item.id() == id);

        index.map(|i| self.collection.remove(i))
    }

    pub fn get(&self, id: &StringBasedId) -> Option<&T> {
        self.collection.iter().find(|item| item.id() == id)
    }

    pub fn get_mut(&mut self, id: &StringBasedId) -> Option<&mut T> {
        self.collection.iter_mut().find(|item| item.id() == id)
    }

    pub fn sort_by_key<F: FnMut(&T) -> O, O: Ord>(&mut self, f: F) {
        self.collection.sort_by_key(f);
    }

    pub fn sort<F: FnMut(&T, &T) -> Ordering>(&mut self, f: F) {
        self.collection.sort_by(f);
    }

    pub fn len(&self) -> usize {
        self.collection.len()
    }

    pub fn is_empty(&self) -> bool {
        self.collection.is_empty()
    }
}
