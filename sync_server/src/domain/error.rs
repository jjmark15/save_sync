use crate::domain::SaveId;

#[derive(Debug, thiserror::Error)]
#[error("failed to store new save: {0}")]
pub(crate) enum StoreNewSaveError {
    InvalidSaveMetadata(#[from] InvalidSaveMetadataError),
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum GetSaveError {
    #[error(transparent)]
    NotFound(#[from] SaveNotFoundError),
}

#[derive(Debug, thiserror::Error, derive_new::new)]
#[error("Save with Id '{0}' not found")]
pub(crate) struct SaveNotFoundError(SaveId);

#[derive(Debug, thiserror::Error)]
#[error("invalid save metadata: {0}")]
pub(crate) enum InvalidSaveMetadataError {
    EmptySaveFileName(#[from] EmptySaveFileNameError),
}

#[derive(Debug, thiserror::Error)]
#[error("save file name cannot be empty")]
pub(crate) struct EmptySaveFileNameError;