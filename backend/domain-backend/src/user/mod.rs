use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

pub mod request;

#[derive(Deserialize, Serialize, FromRow)]
pub struct User {
    pub pk_user_id: i64,
    pub username: String,
}

#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct UserInfo {
    pub pk_user_id: i64,
    pub username: String,
}


