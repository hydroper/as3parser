//! Miscellaneous.

mod arena;
pub use arena::*;

pub use by_address::ByAddress as NodeAsKey;

mod character_reader;
pub use character_reader::*;

mod shared_array;
pub use shared_array::*;

mod shared_map;
pub use shared_map::*;

mod escaping;
pub use escaping::*;

mod css;
pub use css::*;

pub use std::cell::{Cell, RefCell};
pub use std::collections::{HashMap, HashSet};
pub use std::rc::{Rc, Weak};

pub fn default<T: Default>() -> T {
    T::default()
}