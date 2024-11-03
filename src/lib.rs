use std::cell::{Cell, UnsafeCell};
use std::fmt::Debug;
use std::ptr::null;

pub struct HistoryBox<T> {
    pointer: Cell<* const T>,
    history: UnsafeCell<Vec<Box<T>>>,
}

impl<T> HistoryBox<T> {

    pub fn new() -> Self {
        Self {
            pointer: Cell::new(null()),
            history: UnsafeCell::new(Vec::new()),
        }
    }

    pub fn new_with(value: T) -> Self {
        let retval = Self::new();
        retval.set(value);
        retval
    }

    pub fn set(&self, value: T) {
        let history = unsafe { &mut *self.history.get() };
        history.push(Box::new(value));
        self.pointer.set(history.last().unwrap().as_ref());
    }

    pub fn get(&self) -> Option<&T> {
        if self.pointer.get().is_null() {
            None
        } else {
            Some(unsafe { &*self.pointer.get() })
        }
    }
}

impl<T> Default for HistoryBox<T> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<T> Send for HistoryBox<T> where T: Send { }
unsafe impl<T> Sync for HistoryBox<T> where T: Sync { }

impl<T> Debug for HistoryBox<T> where T: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let history = unsafe { &*self.history.get() };
        write!(f, "HistoryBox {{ current: {:?}, history: {:?} }}", self.get(), history)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_history_box() {
        let history_box = HistoryBox::new();
        assert_eq!(history_box.get(), None);
        history_box.set(1);
        assert_eq!(history_box.get(), Some(&1));
        history_box.set(2);
        assert_eq!(history_box.get(), Some(&2));
        history_box.set(3);
        assert_eq!(history_box.get(), Some(&3));
    }

    #[test]
    fn test_history_box_debug_message() {
        let history_box = HistoryBox::new();
        assert_eq!(format!("{:?}", history_box), "HistoryBox { current: None, history: [] }");
        history_box.set(1);
        assert_eq!(format!("{:?}", history_box), "HistoryBox { current: Some(1), history: [1] }");
        history_box.set(2);
        assert_eq!(format!("{:?}", history_box), "HistoryBox { current: Some(2), history: [1, 2] }");
        history_box.set(3);
        assert_eq!(format!("{:?}", history_box), "HistoryBox { current: Some(3), history: [1, 2, 3] }");
    }
}