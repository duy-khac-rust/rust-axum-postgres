use modql::field::Fields;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

pub mod request;

#[derive(Deserialize, Serialize, FromRow, Fields)]
pub struct User {
    pub pk_user_id: i64,
    pub username: String,
}

#[derive(Deserialize, Serialize, FromRow, Clone, Fields)]
pub struct UserInfo {
    pub pk_user_id: i64,
    pub username: String,
}


