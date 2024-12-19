use async_graphql::*;
use entity::entities::{asset, folder};
use sea_orm::{
    entity::*, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect,
};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct FolderType {
    pub id: ID,
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub logo_hash: String,

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
            logo_hash: value.logo_hash,
            client_id: value.client_id,
            date_added: value.date_added.to_string(),
            last_updated: value.last_updated.to_string(),
        }
    }
}

#[ComplexObject]
impl FolderType {
    async fn items_count<'ctx>(&self, ctx: &Context<'ctx>) -> Result<u64> {
        let db = ctx.data::<DatabaseConnection>()?;
        let folder_id = self.id.parse::<i64>()?;
        let count = asset::Entity::find()
            .filter(asset::Column::FolderId.eq(folder_id))
            .count(db)
            .await?;
        Ok(count)
    }

    async fn total_size<'ctx>(&self, ctx: &Context<'ctx>) -> Result<f64> {
        let db = ctx.data::<DatabaseConnection>()?;
        let folder_id = self.id.parse::<i64>()?;
        let size = asset::Entity::find()
            .filter(asset::Column::FolderId.eq(folder_id))
            .select_only()
            .column_as(asset::Column::SizeMb.sum(), "size_sum")
            .into_tuple::<(Option<f64>,)>()
            .one(db)
            .await?;

        if let Some(size) = size {
            if let Some(size) = size.0 {
                Ok(size)
            } else {
                Ok(0.0)
            }
        } else {
            Ok(0.0)
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
    pub content_type: String,
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
            content_type: value.content_type,
            nft_id: value.nft_id,
            client_id: value.client_id,
            folder_id: value.folder_id,
            date_added: value.date_added.to_string(),
            last_updated: value.last_updated.to_string(),
        }
    }
}
