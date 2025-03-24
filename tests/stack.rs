use structured::{
    error::{CapacityError, StackError},
    stack::Stack,
};

const TEST_CAPACITY: usize = 5;

#[test]
fn it_should_not_intialize_stack_with_zero_capacity() {
    let result = Stack::<u16>::new(0);
    assert!(result.is_err());
    let some_error = result.err();
    assert!(some_error.is_some());
    assert_eq!(some_error.unwrap(), CapacityError::Invalid);
}

#[test]
fn it_should_initialize_stack_with_default_capacity() {
    let result = Stack::<u16>::new(TEST_CAPACITY);
    assert!(result.is_ok());
    let some_result = result.ok();
    assert!(some_result.is_some());
    assert!(some_result.unwrap().is_empty());
}

#[test]
fn it_should_not_push_above_capacity() {
    let mut stack = Stack::<u8>::new(TEST_CAPACITY).unwrap();
    for i in 1..=TEST_CAPACITY {
        let _ = stack.push(i as u8);
    }
    let result = stack.push((TEST_CAPACITY + 1) as u8);
    assert!(result.is_err());
    let some_error = result.err();
    assert!(some_error.is_some());
    assert_eq!(some_error.unwrap(), StackError::Overflow);
}

#[test]
fn it_should_not_pop_from_empty_stack() {
    let mut stack = Stack::<u16>::new(TEST_CAPACITY).unwrap();
    for i in 1..=TEST_CAPACITY {
        let _ = stack.push(i as u16);
    }
    for _ in (1..=TEST_CAPACITY).rev() {
        let _ = stack.pop();
    }
    let result = stack.pop();
    assert!(result.is_err());
    let some_error = result.err();
    assert!(some_error.is_some());
    assert_eq!(some_error.unwrap(), StackError::Empty);
}

#[test]
fn it_should_pop_correct_items_from_stack() {
    let mut stack = Stack::<u8>::new(TEST_CAPACITY).unwrap();
    for i in 1..=TEST_CAPACITY {
        let _ = stack.push(i as u8);
    }
    for i in (1..=TEST_CAPACITY).rev() {
        let some_result = stack.pop();
        assert_eq!(*some_result.unwrap(), i as u8);
    }
}
