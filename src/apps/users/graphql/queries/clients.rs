use async_graphql::*;
use entity::entities::{client, subscription_package, user};
use sea_orm::{entity::*, DatabaseConnection, EntityTrait, QueryFilter};

use crate::apps::users::graphql::types::outputs::clients::{ClientType, SubscriptionPackageType};

#[derive(Default)]
pub struct UserClientQueries;

#[Object]
impl UserClientQueries {
    async fn subscription_packages<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<SubscriptionPackageType>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let packages = subscription_package::Entity::find().all(db).await?;
        Ok(packages.into_iter().map(|item| item.into()).collect())
    }

    async fn client<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Option<ClientType>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let user = ctx.data::<Option<user::Model>>()?;

        if let Some(user) = user {
            match client::Entity::find()
                .filter(client::Column::UserId.eq(user.id as i64))
                .one(db)
                .await?
            {
                Some(client) => Ok(Some(client.into())),
                None => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}
