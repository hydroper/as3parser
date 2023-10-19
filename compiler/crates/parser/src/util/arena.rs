use std::{cell::RefCell, rc::{Rc, Weak}};

pub struct Arena<T> {
    data: RefCell<Vec<Rc<T>>>,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self {
            data: RefCell::new(vec![]),
        }
    }

    pub fn alloc(&self, value: T) -> Weak<T> {
        let obj = Rc::new(value);
        self.data.borrow_mut().push(Rc::clone(&obj));
        Rc::downgrade(&obj)
    }
}