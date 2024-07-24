use modql::field::Fields;
use serde::Deserialize;
use sqlx::prelude::FromRow;


#[derive(Deserialize, Fields)]
pub struct RequestGetUser {
    pub id: i64,
}

#[derive(Deserialize, Fields, FromRow)]
pub struct RequestUpdateUser {
    pub pk_user_id: i64,
    pub username: String,
}
