use std::str::FromStr;

use async_graphql::*;
use entity::entities::{asset, client, folder};
use sea_orm::{
    entity::*, DatabaseConnection, EntityTrait, JoinType, QueryFilter, QueryOrder, QuerySelect,
};
use uuid::Uuid;

use crate::apps::{
    assets::graphql::types::{
        inputs::assets::{AssetQueryOptions, FolderQueryOptions},
        outputs::assets::{AssetType, FolderType},
    },
    common::graphql::types::inputs::Paginated,
};

#[derive(Default)]
pub struct AssetQueries;

#[Object]
impl AssetQueries {
    async fn folder_assets<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        folder_id: ID,
        opts: Option<Paginated<AssetQueryOptions>>,
    ) -> Result<Vec<AssetType>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let mut stmt = asset::Entity::find()
            .join(
                JoinType::InnerJoin,
                asset::Entity::belongs_to(folder::Entity)
                    .from(asset::Column::FolderId)
                    .to(folder::Column::Id)
                    .into(),
            )
            .filter(folder::Column::Uuid.eq(Uuid::from_str(folder_id.to_string().as_str())?));

        if let Some(opts) = opts {
            stmt = stmt.offset(opts.offset);
            stmt = stmt.limit(opts.limit);
            if let Some(opts) = opts.opts {
                if let Some(filter) = opts.filter {
                    if let Some(name) = filter.name {
                        stmt = stmt.filter(asset::Column::Name.contains(name));
                    }
                    if let Some(description) = filter.description {
                        stmt = stmt.filter(asset::Column::Description.contains(description));
                    }
                    if let Some(min_size_mb) = filter.min_size_mb {
                        stmt = stmt.filter(asset::Column::SizeMb.gt(min_size_mb));
                    }
                    if let Some(max_size_mb) = filter.max_size_mb {
                        stmt = stmt.filter(asset::Column::SizeMb.lt(max_size_mb));
                    }
                }
                if let Some(ordering) = opts.ordering {
                    if let Some(date_added) = ordering.date_added {
                        if date_added {
                            stmt = stmt.order_by_desc(asset::Column::DateAdded);
                        } else {
                            stmt = stmt.order_by_asc(asset::Column::DateAdded);
                        }
                    }
                    if let Some(last_updated) = ordering.last_updated {
                        if last_updated {
                            stmt = stmt.order_by_desc(asset::Column::LastUpdated);
                        } else {
                            stmt = stmt.order_by_asc(asset::Column::LastUpdated);
                        }
                    }
                }
            }
        }

        let res = stmt.all(db).await?;
        Ok(res.into_iter().map(|item| item.into()).collect())
    }

    async fn folder<'ctx>(&self, ctx: &Context<'ctx>, id: ID) -> Result<FolderType> {
        let db = ctx.data::<DatabaseConnection>()?;
        let folder = folder::Entity::find()
            .filter(folder::Column::Uuid.eq(Uuid::from_str(id.to_string().as_str())?))
            .one(db)
            .await?;

        if let Some(folder) = folder {
            Ok(folder.into())
        } else {
            return Err(Error::new(format!("Folder with uuid {}", &id.to_string())));
        }
    }

    async fn client_folders<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        client_uuid: ID,
        opts: Option<Paginated<FolderQueryOptions>>,
    ) -> Result<Vec<FolderType>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let mut stmt = folder::Entity::find()
            .join(JoinType::InnerJoin, folder::Relation::Client.def())
            .filter(client::Column::Uuid.eq(Uuid::from_str(client_uuid.to_string().as_str())?));

        if let Some(opts) = opts {
            stmt = stmt.limit(opts.limit);
            stmt = stmt.offset(opts.offset);
            if let Some(opts) = opts.opts {
                if let Some(filter) = opts.filter {
                    if let Some(name) = filter.name {
                        stmt = stmt.filter(folder::Column::Name.contains(name));
                    }
                    if let Some(desc) = filter.description {
                        stmt = stmt.filter(folder::Column::Description.contains(desc));
                    }
                }
                if let Some(ordering) = opts.ordering {
                    if let Some(date_added) = ordering.date_added {
                        if date_added {
                            stmt = stmt.order_by_desc(folder::Column::DateAdded);
                        } else {
                            stmt = stmt.order_by_asc(folder::Column::DateAdded);
                        }
                    }
                    if let Some(last_updated) = ordering.last_updated {
                        if last_updated {
                            stmt = stmt.order_by_desc(folder::Column::LastUpdated);
                        } else {
                            stmt = stmt.order_by_asc(folder::Column::LastUpdated);
                        }
                    }
                }
            }
        }
        let res = stmt.all(db).await?;
        Ok(res.into_iter().map(|item| item.into()).collect())
    }
}
