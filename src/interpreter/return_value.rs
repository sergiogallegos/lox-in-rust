use crate::interpreter::value::Object;

#[derive(Debug, Clone)]
pub struct Return {
    pub value: Option<Object>,
}

impl Return {
    /// Creates a new Return exception with a value.
    pub fn new(value: Option<Object>) -> Self {
        Self { value }
    }
}

impl std::fmt::Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Return exception with value: {:?}", self.value)
    }
}

impl std::error::Error for Return {}