mod arena;
pub use arena::*;

pub use by_address::ByAddress as AstAsKey;

mod character_reader;
pub use character_reader::*;

mod shared_array;
pub use shared_array::*;

mod shared_map;
pub use shared_map::*;

pub use std::cell::{Cell, RefCell};
pub use std::collections::{HashMap, HashSet};
pub use std::rc::{Rc, Weak};

pub fn default<T: Default>() -> T {
    T::default()
}