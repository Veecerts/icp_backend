use async_graphql::*;

use crate::apps::assets::graphql::types::inputs::assets::FolderQueryOptions;

#[derive(InputObject)]
#[graphql(concrete(name = "PaginatedFolderQueryOptions", params(FolderQueryOptions)))]
pub struct Paginated<T: InputType> {
    pub opts: Option<T>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}
