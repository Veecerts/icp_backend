use async_graphql::*;
use entity::entities::{client, client_package_subscription, subscription_package};

#[derive(SimpleObject)]
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
