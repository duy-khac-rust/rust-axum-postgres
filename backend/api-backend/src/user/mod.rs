use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use domain_backend::user::{
    request::{RequestGetUser, RequestUpdateUser},
    User, UserInfo,
};

use inftra_backend::user::{
    error::ResultUser, user_create_json, user_delete_by_id, user_get_by_id, user_update_by_id,
    user_view,
};
use sqlx::PgPool;

pub fn user_router() -> Router<PgPool> {
    Router::new()
        .merge(delete_user_route())
        .merge(update_user_route())
        .merge(create_user_route())
        .merge(get_user_route())
        .merge(get_users_route())
}

pub fn get_users_route() -> Router<PgPool> {
    pub async fn get_users(State(db): State<PgPool>) -> ResultUser<Json<Vec<User>>> {
        user_view(db).await
    }

    Router::new().route("/user", get(get_users))
}

pub fn get_user_route() -> Router<PgPool> {
    pub async fn get_user(
        State(db): State<PgPool>,
        Path(id): Path<RequestGetUser>,
    ) -> ResultUser<Json<User>> {
        user_get_by_id(db, id).await
    }

    Router::new().route("/user/:id", get(get_user))
}

pub fn create_user_route() -> Router<PgPool> {
    pub async fn create_user(
        State(db): State<PgPool>,
        Json(user): Json<UserInfo>,
    ) -> ResultUser<Json<UserInfo>> {
        user_create_json(db, user).await
    }

    Router::new().route("/user", post(create_user))
}

pub fn update_user_route() -> Router<PgPool> {
    pub async fn update_user(
        State(db): State<PgPool>,
        Json(user): Json<RequestUpdateUser>,
    ) -> ResultUser<()> {
        user_update_by_id(db, user).await
    }

    Router::new().route("/user/update", put(update_user))
}

pub fn delete_user_route() -> Router<PgPool> {
    pub async fn delete_user(
        State(db): State<PgPool>,
        Path(id): Path<RequestGetUser>,
    ) -> ResultUser<()> {
        user_delete_by_id(db, id.id).await
    }

    Router::new().route("/user/delete/:id", delete(delete_user))
}
