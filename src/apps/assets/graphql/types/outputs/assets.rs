use async_graphql::*;
use entity::entities::{asset, client, folder};
use sea_orm::{
    entity::*, Condition, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect,
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
    pub size_mb: f64,

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
            size_mb: value.size_mb,
            client_id: value.client_id,
            folder_id: value.folder_id,
            date_added: value.date_added.to_string(),
            last_updated: value.last_updated.to_string(),
        }
    }
}

#[derive(SimpleObject)]
pub struct StorageSummary {
    pub count: u64,
    pub total_size: u64,
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct UserFileStorageSummary {
    pub client_id: Option<i32>,
}

#[ComplexObject]
impl UserFileStorageSummary {
    async fn images<'ctx>(&self, ctx: &Context<'ctx>) -> Result<StorageSummary> {
        let db = ctx.data::<DatabaseConnection>()?;
        let mut stmt =
            asset::Entity::find().filter(asset::Column::ContentType.starts_with("image"));

        if let Some(id) = self.client_id {
            stmt = stmt
                .join(sea_orm::JoinType::LeftJoin, asset::Relation::Client2.def())
                .filter(client::Column::Id.eq(id));
        }

        let result = stmt
            .select_only()
            .column_as(asset::Column::Id.count(), "count")
            .column_as(asset::Column::SizeMb.sum(), "total_size")
            .into_tuple::<(Option<u64>, Option<u64>)>()
            .one(db)
            .await?
            .unwrap_or((Some(0), Some(0)));

        Ok(StorageSummary {
            count: result.0.unwrap_or(0),
            total_size: result.1.unwrap_or(0),
        })
    }

    async fn videos<'ctx>(&self, ctx: &Context<'ctx>) -> Result<StorageSummary> {
        let db = ctx.data::<DatabaseConnection>()?;
        let mut stmt =
            asset::Entity::find().filter(asset::Column::ContentType.starts_with("video"));

        if let Some(id) = self.client_id {
            stmt = stmt
                .join(sea_orm::JoinType::LeftJoin, asset::Relation::Client2.def())
                .filter(client::Column::Id.eq(id));
        }

        let result = stmt
            .select_only()
            .column_as(asset::Column::Id.count(), "count")
            .column_as(asset::Column::SizeMb.sum(), "total_size")
            .into_tuple::<(Option<u64>, Option<u64>)>()
            .one(db)
            .await?
            .unwrap_or((Some(0), Some(0)));

        Ok(StorageSummary {
            count: result.0.unwrap_or(0),
            total_size: result.1.unwrap_or(0),
        })
    }

    async fn audios<'ctx>(&self, ctx: &Context<'ctx>) -> Result<StorageSummary> {
        let db = ctx.data::<DatabaseConnection>()?;
        let mut stmt =
            asset::Entity::find().filter(asset::Column::ContentType.starts_with("audio"));

        if let Some(id) = self.client_id {
            stmt = stmt
                .join(sea_orm::JoinType::LeftJoin, asset::Relation::Client2.def())
                .filter(client::Column::Id.eq(id));
        }

        let result = stmt
            .select_only()
            .column_as(asset::Column::Id.count(), "count")
            .column_as(asset::Column::SizeMb.sum(), "total_size")
            .into_tuple::<(Option<u64>, Option<u64>)>()
            .one(db)
            .await?
            .unwrap_or((Some(0), Some(0)));

        Ok(StorageSummary {
            count: result.0.unwrap_or(0),
            total_size: result.1.unwrap_or(0),
        })
    }

    async fn documents<'ctx>(&self, ctx: &Context<'ctx>) -> Result<StorageSummary> {
        let db = ctx.data::<DatabaseConnection>()?;
        let mut stmt = asset::Entity::find().filter(
            Condition::any()
                .add(asset::Column::ContentType.starts_with("application"))
                .add(asset::Column::ContentType.starts_with("text")),
        );

        if let Some(id) = self.client_id {
            stmt = stmt
                .join(sea_orm::JoinType::LeftJoin, asset::Relation::Client2.def())
                .filter(client::Column::Id.eq(id));
        }

        let result = stmt
            .select_only()
            .column_as(asset::Column::Id.count(), "count")
            .column_as(asset::Column::SizeMb.sum(), "total_size")
            .into_tuple::<(Option<u64>, Option<u64>)>()
            .one(db)
            .await?
            .unwrap_or((Some(0), Some(0)));

        Ok(StorageSummary {
            count: result.0.unwrap_or(0),
            total_size: result.1.unwrap_or(0),
        })
    }

    async fn others<'ctx>(&self, ctx: &Context<'ctx>) -> Result<StorageSummary> {
        let db = ctx.data::<DatabaseConnection>()?;
        let mut stmt = asset::Entity::find().filter(
            Condition::any()
                .add(asset::Column::ContentType.starts_with("application"))
                .add(asset::Column::ContentType.starts_with("audio"))
                .add(asset::Column::ContentType.starts_with("video"))
                .add(asset::Column::ContentType.starts_with("image"))
                .add(asset::Column::ContentType.starts_with("text"))
                .not(),
        );

        if let Some(id) = self.client_id {
            stmt = stmt
                .join(sea_orm::JoinType::LeftJoin, asset::Relation::Client2.def())
                .filter(client::Column::Id.eq(id));
        }

        let result = stmt
            .select_only()
            .column_as(asset::Column::Id.count(), "count")
            .column_as(asset::Column::SizeMb.sum(), "total_size")
            .into_tuple::<(Option<u64>, Option<u64>)>()
            .one(db)
            .await?
            .unwrap_or((Some(0), Some(0)));

        Ok(StorageSummary {
            count: result.0.unwrap_or(0),
            total_size: result.1.unwrap_or(0),
        })
    }
}
