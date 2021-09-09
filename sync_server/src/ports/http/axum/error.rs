#[derive(Debug, thiserror::Error, derive_new::new)]
#[error("missing mandatory data")]
pub(crate) struct MissingMandatoryDataError;
