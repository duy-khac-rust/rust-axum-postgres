pub mod error;
pub mod middleware;
pub mod models;
pub mod user;

use error::ResultInftra;
use sqlx::PgPool;

pub async fn init_db(dsn: &str, max_connect: u32) -> ResultInftra<PgPool> {
    let db = sqlx::postgres::PgPoolOptions::new()
        .max_connections(max_connect)
        .connect(dsn)
        .await?;

    sqlx::migrate!().run(&db).await.unwrap();
    Ok(db)
}
