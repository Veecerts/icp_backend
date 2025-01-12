use async_graphql::*;
use entity::entities::{client, client_package_subscription, client_usage, subscription_package};
use sea_orm::{entity::*, DatabaseConnection, EntityTrait, QueryFilter};

use crate::apps::assets::graphql::types::outputs::assets::UserFileStorageSummary;

#[derive(SimpleObject)]
pub struct ClientUsageType {
    pub id: ID,
    pub uuid: String,

    #[graphql(skip)]
    pub client_id: i64,

    pub used_storage_mb: f64,
    pub active_sessions: i32,
    pub date_added: String,
    pub last_updated: String,
}

impl From<client_usage::Model> for ClientUsageType {
    fn from(value: client_usage::Model) -> Self {
        Self {
            id: value.id.into(),
            uuid: value.uuid.to_string(),
            client_id: value.client_id,
            used_storage_mb: value.used_storage_mb,
            active_sessions: value.active_sessions,
            date_added: value.date_added.to_string(),
            last_updated: value.last_updated.to_string(),
        }
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct ClientType {
    pub id: ID,
    pub uuid: String,

    #[graphql(skip)]
    pub user_id: i64,

    #[graphql(skip)]
    pub active_subscription_id: Option<i64>,

    pub api_secret_hash: String,
    pub date_added: String,
    pub last_updated: String,
}

impl From<client::Model> for ClientType {
    fn from(value: client::Model) -> Self {
        Self {
            id: value.id.into(),
            uuid: value.uuid.to_string(),
            user_id: value.user_id,
            active_subscription_id: value.active_subscription_id,
            api_secret_hash: value.api_secret_hash,
            date_added: value.date_added.to_string(),
            last_updated: value.last_updated.to_string(),
        }
    }
}

#[ComplexObject]
impl ClientType {
    async fn usage<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Option<ClientUsageType>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let client_id = self.id.parse::<i64>()?;
        let usage = client_usage::Entity::find()
            .filter(client_usage::Column::ClientId.eq(client_id))
            .one(db)
            .await?;
        if let Some(usage) = usage {
            Ok(Some(usage.into()))
        } else {
            Ok(None)
        }
    }

    async fn file_storage_summary(&self) -> Result<UserFileStorageSummary> {
        let client_id = self.id.parse::<i32>()?;
        Ok(UserFileStorageSummary {
            client_id: Some(client_id),
        })
    }

    async fn active_subscription<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Option<ClientPackageSubscriptionType>> {
        let db = ctx.data::<DatabaseConnection>()?;
        if let Some(active_subscription_id) = self.active_subscription_id {
            let sub =
                client_package_subscription::Entity::find_by_id(active_subscription_id as i32)
                    .one(db)
                    .await?;
            if let Some(sub) = sub {
                return Ok(Some(sub.into()));
            } else {
                return Ok(None);
            };
        }
        Ok(None)
    }
}

#[derive(SimpleObject)]
pub struct SubscriptionPackageType {
    pub id: ID,
    pub uuid: String,
    pub name: String,
    pub price: f64,
    pub storage_capacity_mb: f64,
    pub monthly_requests: i64,
    pub max_allowed_sessions: i32,
    pub date_added: String,
    pub last_updated: String,
}

impl From<subscription_package::Model> for SubscriptionPackageType {
    fn from(value: subscription_package::Model) -> Self {
        Self {
            id: value.id.into(),
            uuid: value.uuid.into(),
            name: value.name,
            price: value.price,
            storage_capacity_mb: value.storage_capacity_mb,
            monthly_requests: value.monthly_requests,
            max_allowed_sessions: value.max_allowed_sessions,
            date_added: value.date_added.to_string(),
            last_updated: value.last_updated.to_string(),
        }
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct ClientPackageSubscriptionType {
    pub id: ID,
    pub uuid: String,
    pub client_id: i64,

    #[graphql(skip)]
    pub subscription_package_id: i64,

    pub amount: f32,
    pub date_added: String,
    pub expires_at: String,
}

impl From<client_package_subscription::Model> for ClientPackageSubscriptionType {
    fn from(value: client_package_subscription::Model) -> Self {
        Self {
            id: value.id.into(),
            uuid: value.uuid.to_string(),
            client_id: value.client_id,
            subscription_package_id: value.subscription_package_id,
            amount: value.amount,
            date_added: value.date_added.to_string(),
            expires_at: value.expires_at.to_string(),
        }
    }
}

#[ComplexObject]
impl ClientPackageSubscriptionType {
    async fn subscription_package<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<SubscriptionPackageType> {
        let db = ctx.data::<DatabaseConnection>()?;
        let package = subscription_package::Entity::find_by_id(self.subscription_package_id as i32)
            .one(db)
            .await?;
        if let Some(package) = package {
            Ok(package.into())
        } else {
            Err(Error::new(format!(
                "Subscription Package for Client with id {} was not found",
                self.client_id
            )))
        }
    }
}
