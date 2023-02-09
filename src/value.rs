#[derive(Debug, Clone, Copy)]
pub enum Value {
    Unit,
    Boolean(bool),
    Integer(i64),
}
