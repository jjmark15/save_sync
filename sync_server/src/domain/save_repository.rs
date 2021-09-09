use crate::domain::{GetSaveError, Save, SaveId, StoreNewSaveError};

#[async_trait::async_trait]
pub(crate) trait SaveRepository {
    async fn store_new(&self, save: Save) -> Result<(), StoreNewSaveError>;

    async fn get(&self, save_id: SaveId) -> Result<Save, GetSaveError>;
}
