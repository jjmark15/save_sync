#[derive(derive_new::new)]
pub(crate) struct GameName {
    value: String,
}

impl GameName {
    pub(crate) fn value(&self) -> &String {
        &self.value
    }
}
