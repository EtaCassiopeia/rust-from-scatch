use std::cell::UnsafeCell;

struct MyCell<T> {
    value: UnsafeCell<T>,
}

impl<T> MyCell<T> {
    pub fn new(value: T) -> Self {
        MyCell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }

    pub fn set(&self, value: T) {
        unsafe { *self.value.get() = value };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_my_cell() {
        let cell = MyCell::new(42);
        assert_eq!(cell.get(), 42);
        cell.set(43);
        assert_eq!(cell.get(), 43);
    }
}
