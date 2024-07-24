use axum::{
    extract::{Path, State},
    Json,
};

use domain_backend::user::{
    request::{RequestGetUser, RequestUpdateUser},
    User, UserInfo,
};

use inftra_backend::base::{
    create, delete_by_id, error::ResultBase, get_by_id, update_by_id, view,
};

use sqlx::PgPool;

use super::UserDmc;

impl UserDmc {
    pub async fn create_user(
        State(db): State<PgPool>,
        Json(user): Json<UserInfo>,
    ) -> ResultBase<Json<UserInfo>> {
        create::<UserDmc, _, _>(db, user).await
    }

    pub async fn get_users(State(db): State<PgPool>) -> ResultBase<Json<Vec<User>>> {
        view::<UserDmc, _>(db).await
    }

    pub async fn get_user(
        State(db): State<PgPool>,
        Path(id): Path<RequestGetUser>,
    ) -> ResultBase<Json<User>> {
        get_by_id::<UserDmc, _>(db, id.id).await
    }

    pub async fn delete_user(
        State(db): State<PgPool>,
        Path(id): Path<RequestGetUser>,
    ) -> ResultBase<()> {
        delete_by_id::<UserDmc>(db, id.id).await
    }

    pub async fn update_user(
        State(db): State<PgPool>,
        Path(id): Path<i64>,
        Json(user): Json<RequestUpdateUser>,
    ) -> ResultBase<()> {
        update_by_id::<UserDmc, _>(db, user, id).await
    }
}
