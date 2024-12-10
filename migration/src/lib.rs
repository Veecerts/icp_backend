pub use sea_orm_migration::prelude::*;
pub mod utils;

mod m20241204_111833_create_user_tables;
mod m20241204_122105_create_client_and_package_tables;
mod m20241205_070110_create_asset_table;
mod m20241205_081228_create_auth_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241204_111833_create_user_tables::Migration),
            Box::new(m20241204_122105_create_client_and_package_tables::Migration),
            Box::new(m20241205_070110_create_asset_table::Migration),
            Box::new(m20241205_081228_create_auth_tables::Migration),
        ]
    }
}
