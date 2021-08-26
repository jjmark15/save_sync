use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub struct SaveId {
    value: String,
}

impl SaveId {
    pub fn new(value: String) -> Self {
        SaveId { value }
    }
}

impl Display for SaveId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for SaveId {
    fn from(s: &str) -> Self {
        SaveId::new(s.to_string())
    }
}
