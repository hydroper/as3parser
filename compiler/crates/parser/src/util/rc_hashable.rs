use std::{rc::Rc, hash::Hash};

/// Work with a `Rc<T>` value by its memory reference address.
/// The `Hash`, `Clone` and `PartialEq` traits implemented
/// by this structure operate on the pointer rather than the
/// content.
pub struct RcHashable<T>(pub Rc<T>);

impl<T> Hash for RcHashable<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.0).hash(state)
    }
}

impl<T> Clone for RcHashable<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T> Eq for RcHashable<T> {}

impl<T> PartialEq for RcHashable<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}