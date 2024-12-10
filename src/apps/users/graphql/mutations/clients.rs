use async_graphql::*;
use chrono::Utc;
use entity::entities::subscription_package;
use sea_orm::{entity::*, DatabaseConnection, EntityTrait, Set};

use crate::apps::users::graphql::types::{
    inputs::clients::SubscriptionPackageInput, outputs::clients::SubscriptionPackageType,
};

#[derive(Default)]
pub struct UserClientMutations;

#[Object]
impl UserClientMutations {
    async fn create_update_subscription_package<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: SubscriptionPackageInput,
    ) -> Result<SubscriptionPackageType> {
        let db = ctx.data::<DatabaseConnection>()?;
        if let Some(id) = input.id {
            let package = subscription_package::Entity::find_by_id(id.parse::<i32>()?)
                .one(db)
                .await?;
            if let Some(package) = package {
                let mut package: subscription_package::ActiveModel = package.into();
                package.name = Set(input.name);
                package.price = Set(input.price);
                package.storage_capacity_mb = Set(input.storage_capacity_mb);
                package.monthly_requests = Set(input.monthly_requests);
                package.max_allowed_sessions = Set(input.max_allowed_sessions);
                package.last_updated = Set(Utc::now().naive_utc());

                let package: subscription_package::Model = package.update(db).await?;
                Ok(package.into())
            } else {
                return Err(Error::new(format!(
                    "SubscriptionPackage with id {} not found",
                    &id.to_string()
                )));
            }
        } else {
            let package = subscription_package::ActiveModel {
                name: Set(input.name),
                price: Set(input.price),
                storage_capacity_mb: Set(input.storage_capacity_mb),
                monthly_requests: Set(input.monthly_requests),
                max_allowed_sessions: Set(input.max_allowed_sessions),
                last_updated: Set(Utc::now().naive_utc()),
                ..Default::default()
            };
            let package: subscription_package::Model = package.insert(db).await?;
            Ok(package.into())
        }
    }
}
