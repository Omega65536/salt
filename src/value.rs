#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Unit,
    Boolean(bool),
    Integer(i64),
}
