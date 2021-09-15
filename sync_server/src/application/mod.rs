use std::convert::TryFrom;

use uuid::Uuid;

use crate::domain::{
    GameName, GetSaveError, InvalidSaveMetadataError, Save, SaveData, SaveFileName, SaveId,
    SaveService, StoreNewSaveError,
};

#[async_trait::async_trait]
pub(crate) trait ApplicationService {
    async fn store_new_save(
        &self,
        game_name: String,
        file_name: String,
        save_data: Vec<u8>,
    ) -> Result<Uuid, StoreNewSaveError>;

    async fn get_save(&self, save_id: Uuid) -> Result<Save, GetSaveError>;

    async fn get_latest_version(&self, save_id: Uuid) -> Result<u32, GetSaveError>;
}

pub(crate) struct ApplicationServiceImpl<SS: SaveService> {
    save_service: SS,
}

#[async_trait::async_trait]
impl<SS> ApplicationService for ApplicationServiceImpl<SS>
where
    SS: SaveService + Send + Sync,
{
    async fn store_new_save(
        &self,
        game_name: String,
        file_name: String,
        save_data: Vec<u8>,
    ) -> Result<Uuid, StoreNewSaveError> {
        self.save_service
            .store_new(
                GameName::new(game_name),
                SaveData::new(save_data),
                SaveFileName::try_from(file_name).map_err(InvalidSaveMetadataError::from)?,
            )
            .await
            .map(|save_id| *save_id.value())
    }

    async fn get_save(&self, save_id: Uuid) -> Result<Save, GetSaveError> {
        self.save_service.get(SaveId::new(save_id)).await
    }

    async fn get_latest_version(&self, save_id: Uuid) -> Result<u32, GetSaveError> {
        self.save_service
            .get_latest_version(SaveId::new(save_id))
            .await
            .map(|version| version.value())
    }
}

impl<SS: SaveService> ApplicationServiceImpl<SS> {
    pub(crate) fn new(save_service: SS) -> Self {
        ApplicationServiceImpl { save_service }
    }
}
