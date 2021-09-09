use std::fmt::{Display, Formatter};

use uuid::Uuid;

#[derive(Debug, derive_new::new, Copy, Clone)]
pub(crate) struct SaveId {
    value: Uuid,
}

impl SaveId {
    pub(crate) fn value(&self) -> &Uuid {
        &self.value
    }
}

impl Display for SaveId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
