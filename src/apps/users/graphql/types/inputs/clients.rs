use async_graphql::*;

#[derive(InputObject)]
pub struct SubscriptionPackageInput {
    pub id: Option<ID>,
    pub name: String,
    pub price: f32,
    pub storage_capacity_mb: i64,
    pub monthly_requests: i64,
    pub max_allowed_sessions: i32,
    pub date_added: String,
    pub last_update: String,
}
