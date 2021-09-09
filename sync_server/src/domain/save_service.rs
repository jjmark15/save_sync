use uuid::Uuid;

use crate::domain::{
    GameName, GetSaveError, Save, SaveData, SaveFileName, SaveId, SaveRepository, SaveVersion,
    StoreNewSaveError,
};

#[async_trait::async_trait]
pub(crate) trait SaveService {
    async fn store_new(
        &self,
        game_name: GameName,
        save_data: SaveData,
        save_file_name: SaveFileName,
    ) -> Result<SaveId, StoreNewSaveError>;

    async fn get(&self, save_id: SaveId) -> Result<Save, GetSaveError>;

    async fn get_latest_version(&self, save_id: SaveId) -> Result<SaveVersion, GetSaveError>;
}

pub(crate) struct SaveServiceImpl<SR: SaveRepository> {
    save_repository: SR,
}

impl<SR> SaveServiceImpl<SR>
where
    SR: SaveRepository + Sync + Send,
{
    pub(crate) fn new(save_repository: SR) -> Self {
        SaveServiceImpl { save_repository }
    }

    fn generate_new_game_save_id(&self) -> SaveId {
        SaveId::new(Uuid::new_v4())
    }
}

#[async_trait::async_trait]
impl<SR> SaveService for SaveServiceImpl<SR>
where
    SR: SaveRepository + Send + Sync,
{
    async fn store_new(
        &self,
        game_name: GameName,
        save_data: SaveData,
        save_file_name: SaveFileName,
    ) -> Result<SaveId, StoreNewSaveError> {
        let save_id = self.generate_new_game_save_id();
        let save = Save::new(
            save_id,
            SaveVersion::new(0),
            game_name,
            save_file_name,
            save_data,
        );
        self.save_repository.store_new(save).await?;
        Ok(save_id)
    }

    async fn get(&self, save_id: SaveId) -> Result<Save, GetSaveError> {
        self.save_repository.get(save_id).await
    }

    async fn get_latest_version(&self, save_id: SaveId) -> Result<SaveVersion, GetSaveError> {
        Ok(self.save_repository.get(save_id).await?.version())
    }
}
