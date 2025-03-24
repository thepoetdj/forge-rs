#[derive(Debug, PartialEq)]
pub enum StackError {
    Overflow,
    Empty,
}

#[derive(Debug, PartialEq)]
pub enum CapacityError {
    Invalid,
}
