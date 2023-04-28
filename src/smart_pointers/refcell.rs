use crate::smart_pointers::cell::MyCell;
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};

#[derive(Copy, Clone)]
enum RefState {
    Unshared,
    Exclusive,
    Shared(usize),
}

struct RefCell<T> {
    value: UnsafeCell<T>,
    state: MyCell<RefState>,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        RefCell {
            value: UnsafeCell::new(value),
            state: MyCell::new(RefState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                Some(Ref { refcell: self })
            }
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                Some(Ref { refcell: self })
            }
            RefState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Exclusive);
                Some(RefMut { refcell: self })
            }
            _ => None,
        }
    }
}

struct Ref<'a, T> {
    refcell: &'a RefCell<T>,
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        eprintln!("Dropping Ref");
        match self.refcell.state.get() {
            RefState::Shared(1) => self.refcell.state.set(RefState::Unshared),
            RefState::Shared(n) => self.refcell.state.set(RefState::Shared(n - 1)),
            _ => unreachable!(),
        }
    }
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

struct RefMut<'a, T> {
    refcell: &'a RefCell<T>,
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        self.refcell.state.set(RefState::Unshared);
    }
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.refcell.value.get() }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_refcell() {
        use super::*;
        let cell: RefCell<i32> = RefCell::new(42);
        let value: Option<Ref<i32>> = cell.borrow();
        assert!(value.is_some());
        let value: Ref<i32> = value.unwrap();
        assert_eq!(*value, 42);

        // should fail to borrow mutably because there is already a shared reference borrowed
        let value1: Option<RefMut<i32>> = cell.borrow_mut();
        assert!(value1.is_none());

        // drop the shared reference
        drop(value);

        let value: Option<RefMut<i32>> = cell.borrow_mut();
        assert!(value.is_some());
        let value: RefMut<i32> = value.unwrap();
        assert_eq!(*value, 42);
    }
}
