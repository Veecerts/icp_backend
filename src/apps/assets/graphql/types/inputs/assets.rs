use async_graphql::*;

#[derive(InputObject)]
pub struct AssetInput {
    pub uuid: Option<ID>,
    pub name: String,
    pub folder_id: i64,
    pub description: String,
    pub file: Upload,
}

#[derive(InputObject)]
pub struct FolderInput {
    pub uuid: Option<ID>,
    pub name: String,
    pub description: String,
    pub logo: Upload,
}

#[derive(InputObject)]
pub struct FolderFilter {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(InputObject)]
pub struct FolderOrdering {
    pub date_added: Option<bool>,
    pub last_updated: Option<bool>,
}

#[derive(InputObject)]
pub struct FolderQueryOptions {
    pub filter: Option<FolderFilter>,
    pub ordering: Option<FolderOrdering>,
}
