use std::convert::TryFrom;
use std::path::PathBuf;

use kv::{Bucket, Config, Json, Store};

use crate::domain::{
    GameName, GetSaveError, Save, SaveData, SaveFileName, SaveId, SaveNotFoundError,
    SaveRepository, SaveVersion, StoreNewSaveError,
};
use crate::ports::persistence::kv::kv_save::KvSave;

pub(crate) struct KvSaveRepositoryAdapter {
    store: Store,
}

impl KvSaveRepositoryAdapter {
    pub(crate) fn new(store_directory: PathBuf) -> Self {
        let cfg = Config::new(store_directory);
        let store = Store::new(cfg).unwrap();

        KvSaveRepositoryAdapter { store }
    }

    fn save_bucket(&self) -> Bucket<String, Json<KvSave>> {
        self.store.bucket(None).unwrap()
    }
}

#[async_trait::async_trait]
impl SaveRepository for KvSaveRepositoryAdapter {
    async fn store_new(&self, save: Save) -> Result<(), StoreNewSaveError> {
        let save_id = save.id().value().to_string();
        let kv_save = Json(KvSave::new(
            save.version().value(),
            save.game_name().value().to_string(),
            save.file_name().value().to_string(),
            save.data().value(),
        ));
        self.save_bucket().set(save_id, kv_save).unwrap();

        Ok(())
    }

    async fn get(&self, save_id: SaveId) -> Result<Save, GetSaveError> {
        let record = self
            .save_bucket()
            .get(save_id.value().to_string())
            .map_err(|_e| SaveNotFoundError::new(save_id))?
            .ok_or_else(|| SaveNotFoundError::new(save_id))?
            .0;

        Ok(Save::new(
            save_id,
            SaveVersion::new(record.save_version()),
            GameName::new(record.game_name().to_string()),
            SaveFileName::try_from(record.file_name().to_string()).unwrap(),
            SaveData::new(record.save_data()),
        ))
    }
}
