#[derive(derive_new::new, Copy, Clone)]
pub(crate) struct SaveVersion {
    value: u32,
}

impl SaveVersion {
    pub(crate) fn value(&self) -> u32 {
        self.value
    }
}
