use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

struct EntangledBit {
    value: Rc<RefCell<bool>>,
}

impl Default for EntangledBit {
    fn default() -> Self {
        Self { value: false }
    }
}

impl EntangledBit {
    fn entangle_with(&self, other: &mut Self) {}
}
pub fn main_es6() {}
