use async_graphql::*;
use entity::entities::subscription_package;

#[derive(SimpleObject)]
pub struct SubscriptionPackageType {
    pub id: ID,
    pub uuid: String,
    pub name: String,
    pub price: f32,
    pub storage_capacity_mb: i64,
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
