use std::str::FromStr;

use async_graphql::*;
use entity::entities::{client, folder};
use sea_orm::{
    entity::*, DatabaseConnection, EntityTrait, JoinType, QueryFilter, QueryOrder, QuerySelect,
};
use uuid::Uuid;

use crate::apps::{
    assets::graphql::types::{inputs::assets::FolderQueryOptions, outputs::assets::FolderType},
    common::graphql::types::inputs::Paginated,
};

#[derive(Default)]
pub struct AssetQueries;

#[Object]
impl AssetQueries {
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
