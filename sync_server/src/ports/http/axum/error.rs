#[derive(Debug, thiserror::Error, derive_new::new)]
#[error("missing mandatory data")]
pub(crate) struct MissingMandatoryDataError;

#[derive(Debug, thiserror::Error, derive_new::new)]
#[error("save file name must not be empty")]
pub(crate) struct EmptySaveFileNameError;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub(crate) enum StoreNewSaveError {
    MissingMandatoryData(#[from] MissingMandatoryDataError),
    EmptySaveFileName(#[from] EmptySaveFileNameError),
    Application(#[from] crate::domain::StoreNewSaveError),
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub(crate) enum GetSaveError {
    Application(#[from] crate::domain::GetSaveError),
}
