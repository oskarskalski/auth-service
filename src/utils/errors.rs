#[derive(Debug)]
pub enum DatabaseErrors {
    NotFound(String),
    CannotInsertData(String)
}
#[derive(Debug)]
pub enum ValidationErrors {
    NullValue(String),
    InvalidValue(String),
    NotFound(String)
}

impl ValidationErrors {
    pub fn message(&self) -> &str {
        match self {
            ValidationErrors::NullValue(msg) => msg,
            ValidationErrors::InvalidValue(msg) => msg,
            ValidationErrors::NotFound(msg) => msg,
        }
    }
}