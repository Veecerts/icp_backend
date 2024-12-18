use async_graphql::*;
use entity::entities::{asset, folder};

#[derive(SimpleObject)]
pub struct FolderType {
    pub id: ID,
    pub uuid: String,
    pub name: String,
    pub description: String,

    #[graphql(skip)]
    pub client_id: i64,

    pub date_added: String,
    pub last_updated: String,
}

impl From<folder::Model> for FolderType {
    fn from(value: folder::Model) -> Self {
        Self {
            id: value.id.into(),
            uuid: value.uuid.to_string(),
            name: value.name,
            description: value.description,
            client_id: value.client_id,
            date_added: value.date_added.to_string(),
            last_updated: value.last_updated.to_string(),
        }
    }
}

#[derive(SimpleObject)]
pub struct AssetType {
    pub id: ID,
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub ipfs_hash: String,
    pub nft_id: i64,

    #[graphql(skip)]
    pub client_id: i64,

    #[graphql(skip)]
    pub folder_id: i64,

    pub date_added: String,
    pub last_updated: String,
}

impl From<asset::Model> for AssetType {
    fn from(value: asset::Model) -> Self {
        Self {
            id: value.id.into(),
            uuid: value.uuid.to_string(),
            name: value.name,
            description: value.description,
            ipfs_hash: value.ipfs_hash,
            nft_id: value.nft_id,
            client_id: value.client_id,
            folder_id: value.folder_id,
            date_added: value.date_added.to_string(),
            last_updated: value.last_updated.to_string(),
        }
    }
}
