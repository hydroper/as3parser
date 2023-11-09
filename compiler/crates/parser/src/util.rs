mod arena;
pub use arena::*;

mod by_address;
pub use by_address::*;

mod code_points_reader;
pub use code_points_reader::*;

pub fn default<T: Default>() -> T {
    T::default()
}