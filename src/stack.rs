use crate::error::{CapacityError, StackError};

pub struct Stack<T> {
    items: Vec<T>,
    top: usize,
}

impl<T> Stack<T> {
    pub fn new(capacity: usize) -> Result<Self, CapacityError> {
        if capacity == 0 {
            return Err(CapacityError::Invalid);
        }
        Ok(Stack {
            items: Vec::<T>::with_capacity(capacity),
            top: 0,
        })
    }

    pub fn push(&mut self, item: T) -> Result<usize, StackError> {
        if self.items.len() == self.items.capacity() {
            return Err(StackError::Overflow);
        }
        self.items.push(item);
        self.top = self.items.len();
        Ok(self.top)
    }

    pub fn pop(&mut self) -> Result<&T, StackError> {
        if self.top == 0 {
            return Err(StackError::Empty);
        }
        let some_item = self.items.get(self.top - 1).unwrap();
        self.top -= 1;
        Ok(some_item)
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
