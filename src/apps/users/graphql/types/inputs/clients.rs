use async_graphql::*;

#[derive(InputObject)]
pub struct SubscriptionPackageInput {
    pub uuid: Option<ID>,
    pub name: String,
    pub price: f64,
    pub storage_capacity_mb: f64,
    pub monthly_requests: i64,
    pub max_allowed_sessions: i32,
}

#[derive(InputObject)]
pub struct ClientPackageSubscriptionInput {
    pub uuid: Option<ID>,
    pub subscription_package_uuid: String,
}
