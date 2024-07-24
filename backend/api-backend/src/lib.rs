use axum::Router;
use sqlx::PgPool;
use user::user_routes;

pub mod error;
pub mod user;

pub fn routes() -> Router<PgPool> {
    Router::new().nest(
        "/api/v1", 
        Router::new()
                     .merge(user_routes())
    )
}

// region:      --- Modules
/*
mod ...
    */

// -- Flatten
/*
pub use ...::*;
    */

//  endregion: --- Modules
