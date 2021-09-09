use std::convert::TryFrom;

use crate::domain::EmptySaveFileNameError;

pub(crate) struct SaveFileName {
    value: String,
}

impl SaveFileName {
    fn new(value: String) -> Self {
        SaveFileName { value }
    }

    pub(crate) fn value(&self) -> &String {
        &self.value
    }
}

impl TryFrom<String> for SaveFileName {
    type Error = EmptySaveFileNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            Err(EmptySaveFileNameError)
        } else {
            Ok(SaveFileName::new(value))
        }
    }
}
