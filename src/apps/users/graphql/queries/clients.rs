use async_graphql::*;
use entity::entities::subscription_package;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::apps::users::graphql::types::outputs::clients::SubscriptionPackageType;

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
        return Ok(packages.into_iter().map(|item| item.into()).collect());
    }
}
