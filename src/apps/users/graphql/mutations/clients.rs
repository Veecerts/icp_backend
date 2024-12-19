use async_graphql::*;
use chrono::Utc;
use entity::entities::{client, client_package_subscription, subscription_package, user};
use sea_orm::{entity::*, DatabaseConnection, EntityTrait, QueryFilter, Set};
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    str::FromStr,
};
use uuid::Uuid;

use crate::apps::users::graphql::types::{
    inputs::clients::{ClientPackageSubscriptionInput, SubscriptionPackageInput},
    outputs::clients::{ClientPackageSubscriptionType, SubscriptionPackageType},
};

#[derive(Default)]
pub struct UserClientMutations;

#[Object]
impl UserClientMutations {
    async fn create_update_client_package_subscription<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: ClientPackageSubscriptionInput,
    ) -> Result<ClientPackageSubscriptionType> {
        let db = ctx.data::<DatabaseConnection>()?;
        let user = ctx.data::<Option<user::Model>>()?;

        if let Some(user) = user {
            let package = subscription_package::Entity::find()
                .filter(
                    subscription_package::Column::Uuid
                        .eq(Uuid::from_str(&input.subscription_package_uuid)?),
                )
                .one(db)
                .await?;
            if let Some(package) = package {
                let client = client::Entity::find()
                    .filter(client::Column::UserId.eq(user.id))
                    .one(db)
                    .await?;

                let client = if let Some(client) = client {
                    client
                } else {
                    let uuid = Uuid::new_v4();
                    let mut hasher = DefaultHasher::new();
                    let uuid_str = uuid.to_string();
                    uuid_str.hash(&mut hasher);
                    let api_secret = format!("{:x}", hasher.finish());

                    let client = client::ActiveModel {
                        uuid: Set(uuid),
                        user_id: Set(user.id as i64),
                        active_subscription_id: Set(package.id as i64),
                        api_secret_hash: Set(api_secret),
                        ..Default::default()
                    };
                    client.insert(db).await?
                };
                let client_package = client_package_subscription::ActiveModel {
                    uuid: Set(Uuid::new_v4()),
                    client_id: Set(client.id as i64),
                    subscription_package_id: Set(package.id as i64),
                    amount: Set(package.price as f32),
                    ..Default::default()
                };

                let client_package = client_package.insert(db).await?;
                Ok(client_package.into())
            } else {
                Err(Error::new(format!(
                    "SubscriptionPackage with uuid {} was not found",
                    &input.subscription_package_uuid
                )))
            }
        } else {
            return Err(Error::new(
                "You must be authenticated to perform this action",
            ));
        }
    }
    async fn create_update_subscription_package<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: SubscriptionPackageInput,
    ) -> Result<SubscriptionPackageType> {
        let db = ctx.data::<DatabaseConnection>()?;
        if let Some(uuid) = input.uuid {
            let package = subscription_package::Entity::find()
                .filter(
                    subscription_package::Column::Uuid
                        .eq(Uuid::from_str(uuid.to_string().as_str())?),
                )
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
                    &uuid.to_string()
                )));
            }
        } else {
            let package = subscription_package::ActiveModel {
                uuid: Set(Uuid::new_v4()),
                name: Set(input.name),
                price: Set(input.price),
                storage_capacity_mb: Set(input.storage_capacity_mb),
                monthly_requests: Set(input.monthly_requests),
                max_allowed_sessions: Set(input.max_allowed_sessions),
                ..Default::default()
            };
            let package: subscription_package::Model = package.insert(db).await?;
            Ok(package.into())
        }
    }
}
