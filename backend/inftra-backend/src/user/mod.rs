pub mod error;
// use axum::Json;
// use domain_backend::user::{
//     request::{RequestGetUser, RequestUpdateUser},
//     User,
// };
// use error::ResultUser;
// use sqlx::PgPool;

// pub mod error;

// pub async fn user_get_by_id(db: PgPool, id: RequestGetUser) -> ResultUser<Json<User>> {
//     let user =
//         sqlx::query_as::<_, User>(r#"SELECT * FROM "user"."tbl_user" WHERE pk_user_id = $1"#)
//             .bind(id.id)
//             .fetch_optional(&db)
//             .await?;

//     Ok(Json(user.unwrap()))
// }

// // pub async fn user_update_by_id(db: PgPool, req: RequestUpdateUser) -> ResultUser<()> {
// //     let count = sqlx::query(r#"UPDATE "user"."tbl_user" SET username =$2 WHERE pk_user_id = $1 "#)
// //         .bind(req.id)
// //         .bind(req.username)
// //         .execute(&db)
// //         .await?
// //         .rows_affected();

// //     if count == 0 {
// //         return Err(error::ErrorUser::NotFound);
// //     }

// //     Ok(())
// // }

// pub async fn user_delete_by_id(db: PgPool, id: i64) -> ResultUser<()> {
//     let count = sqlx::query(r#"DELETE FROM "user"."tbl_user" WHERE pk_user_id = $1"#)
//         .bind(id)
//         .execute(&db)
//         .await?
//         .rows_affected();

//     if count == 0 {
//         return Err(error::ErrorUser::NotFound);
//     }

//     Ok(())
// }
