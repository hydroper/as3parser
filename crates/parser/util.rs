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

/// Counts the first whitespace characters of a string.
pub fn count_first_whitespace_characters(input: &str) -> usize {
    input.chars().count() - input.trim_start().chars().count()
}

/// Decreases the last offset of a range without ever going behind the first offset.
pub fn decrease_last_offset(first_offset: usize, mut last_offset: usize, count: usize) -> usize {
    for _ in 0..count {
        last_offset = if first_offset < last_offset { last_offset - 1 } else { last_offset };
    }
    last_offset
}