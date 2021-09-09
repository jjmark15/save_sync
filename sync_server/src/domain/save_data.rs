#[derive(derive_new::new)]
pub(crate) struct SaveData {
    value: Vec<u8>,
}

impl SaveData {
    pub(crate) fn value(self) -> Vec<u8> {
        self.value
    }
}
