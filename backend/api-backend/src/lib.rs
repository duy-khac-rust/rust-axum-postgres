use axum::Router;
use sqlx::PgPool;
use user::user_router;


pub mod error;
pub mod user;

pub fn routes() -> Router<PgPool> {
    Router::new().nest(
        "/api/v1", 
        Router::new()
        .merge(user_router())
    )
}
