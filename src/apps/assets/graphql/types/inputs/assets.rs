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
    pub logo: Option<Upload>,
}
