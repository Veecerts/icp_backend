use async_graphql::*;
use sea_orm::DatabaseConnection;

use crate::apps::{
    assets::graphql::{mutations::assets::AssetMutations, queries::assets::AssetQueries},
    users::graphql::{
        mutations::{auth::UsersAuthMutations, clients::UserClientMutations},
        queries::{clients::UserClientQueries, users::UserQueries},
    },
};

#[derive(MergedObject, Default)]
pub struct Query(UserQueries, UserClientQueries, AssetQueries);

#[derive(MergedObject, Default)]
pub struct Mutation(UsersAuthMutations, UserClientMutations, AssetMutations);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn get_schema(db_conn: DatabaseConnection) -> AppSchema {
    AppSchema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db_conn)
        .finish()
}
