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

impl From<String> for SaveFileName {
    fn from(value: String) -> Self {
        SaveFileName::new(value)
    }
}
