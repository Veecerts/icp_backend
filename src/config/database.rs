use sea_orm::*;

use super::settings::ENV;

pub async fn connect_db() -> Result<DatabaseConnection, DbErr> {
    let db_url = ENV::init().db_url;
    let mut opts = ConnectOptions::new(db_url);
    opts.sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    Database::connect(opts).await
}
