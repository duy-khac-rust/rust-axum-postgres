use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;

use super::UserDmc;

pub fn user_routes() -> Router<PgPool> {
    Router::new()
        .route("/user", post(UserDmc::create_user))
        .route("/user", get(UserDmc::get_user))
        .route("/user/:id", get(UserDmc::get_users))
        .route("/user/delete/:id", delete(UserDmc::delete_user))
        .route("/user/update/:id", put(UserDmc::update_user))
}
