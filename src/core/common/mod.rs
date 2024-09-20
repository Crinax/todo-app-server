use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

pub mod rules;

pub trait AsShared {
    fn as_shared(self) -> Rc<RefCell<Self>>;
    fn as_atomic_shared(self) -> Arc<Mutex<Self>>;
}

// for test commit
// TODO: Сначала переделать на воркспейсы
// #[proc_macro_derive(AsShared)]
// pub fn derive_as_shared(input: TokenStreamсф) -> TokenStream {
    
// }