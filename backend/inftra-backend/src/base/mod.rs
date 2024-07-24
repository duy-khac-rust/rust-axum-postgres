use axum::Json;
use error::ResultBase;
use modql::{field::HasFields, SIden};
use sea_query::{Asterisk, Expr, Iden, IntoIden, PostgresQueryBuilder, Query, SeaRc, TableRef};

use sea_query_binder::SqlxBinder;
use sqlx::{postgres::PgRow, query_as_with, FromRow, PgPool};

pub mod error;

pub trait DMC {
    const SCHEMA: &'static str;
    const TABLE: &'static str;
    const PRIMARY_KEY_ID: &'static str;

    fn table_ref() -> TableRef {
        TableRef::SchemaTable(
            SIden(Self::SCHEMA).into_iden(),
            SIden(Self::TABLE).into_iden(),
        )
    }

    fn primary_key_id() -> SIden {
        SIden(Self::PRIMARY_KEY_ID)
    }

    
}

pub async fn create<MC, I, O>(db: PgPool, input: I) -> ResultBase<Json<O>>
where
    MC: DMC,
    I: HasFields,
    O: HasFields + for<'a> FromRow<'a, PgRow> + Send + Unpin,
{
    // 1. Setup data
    let fields = input.not_none_fields();
    let (columns, sea_value) = fields.for_sea_insert();

    // 2. Preparing Query
    let mut query = Query::insert();
    query
        .into_table(MC::table_ref())
        .columns(columns)
        .values(sea_value)
        .expect("msg")
        .returning(Query::returning().columns([Asterisk]));

    // 3. Execute
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let entity = query_as_with::<_, O, _>(&sql, values)
        .fetch_one(&db)
        .await?;

    Ok(axum::Json(entity))
}

pub async fn view<MC, O>(db: PgPool) -> ResultBase<Json<Vec<O>>>
where
    MC: DMC,
    O: HasFields + for<'a> FromRow<'a, PgRow> + Send + Unpin,
{
    // 1. Preparing Query
    let mut query = Query::select();
    query.from(MC::table_ref()).columns([Asterisk]);

    // 2. Execute
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let entities = query_as_with::<_, O, _>(&sql, values)
        .fetch_all(&db)
        .await?;

    Ok(axum::Json(entities))
}

pub async fn get_by_id<MC, O>(db: PgPool, id: i64) -> ResultBase<Json<O>>
where
    MC: DMC,
    O: HasFields + for<'a> FromRow<'a, PgRow> + Send + Unpin,
{
    // 1. Preparing Query
    let mut query = Query::select();
    query
        .from(MC::table_ref())
        .and_where(Expr::col(MC::primary_key_id()).eq(id))
        .columns([Asterisk]);

    // 2. Execute
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let entity = query_as_with::<_, O, _>(&sql, values)
        .fetch_one(&db)
        .await?;

    Ok(axum::Json(entity))
}

pub async fn delete_by_id<MC>(db: PgPool, id: i64) -> ResultBase<()>
where
    MC: DMC,
{
    // 1. Preparing Query
    let mut query = Query::delete();
    query
        .from_table(MC::table_ref())
        .and_where(Expr::col(MC::primary_key_id()).eq(id))
        .returning(Query::returning().columns([MC::primary_key_id()]));

    // 2. Execute
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let result = sqlx::query_with(&sql, values)
        .execute(&db)
        .await?
        .rows_affected();

    if result == 0 {
        return Err(error::ErrorBase::NotFound);
    }

    Ok(())
}

pub async fn update_by_id<MC, I>(db: PgPool, input: I, id: i64) -> ResultBase<()>
where
    MC: DMC,
    I: HasFields,
{
    // 1. Setup Data
    let fields = input.not_none_fields();
    let update_pairs = fields.for_sea_update(); // Expecting an iterator

    // Convert iterator to Vec
    let update_values: Vec<(SeaRc<dyn Iden>, sea_query::SimpleExpr)> = update_pairs.collect();

    // 2. Preparing Query
    let mut query = Query::update();
    query
        .table(MC::table_ref())
        .values(update_values.into_iter()) // Ensure this is an iterator
        .and_where(Expr::col(MC::primary_key_id()).eq(id));

    // 3. Execute Query
    let (sql, sql_values) = query.build_sqlx(PostgresQueryBuilder);
    let result = sqlx::query_with(&sql, sql_values)
        .execute(&db)
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}


