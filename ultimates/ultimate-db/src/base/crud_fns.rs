use modql::field::{HasSeaFields, SeaField, SeaFields};
use modql::filter::{FilterGroups, ListOptions};
use sea_query::{Condition, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use sqlx::Row;

use crate::base::{prep_fields_for_create, prep_fields_for_update, CommonIden, DbBmc};
use crate::{Error, Result};
use crate::{Id, ModelManager};

use super::check_number_of_affected;

/// Create a new entity。需要自增主键ID
pub async fn create<MC, E>(mm: &ModelManager, data: E) -> Result<i64>
where
    MC: DbBmc,
    E: HasSeaFields,
{
    let ctx = mm.ctx_ref()?;
    // -- Extract fields (name / sea-query value expression)
    let mut fields = data.not_none_sea_fields();
    fields = SeaFields::new(fields.into_iter().filter(|f| f.iden.to_string() != "id").collect());
    fields = prep_fields_for_create::<MC>(fields, ctx);

    // -- Build query
    let (columns, sea_values) = fields.for_sea_insert();
    let mut query = Query::insert();
    query
        .into_table(MC::table_ref())
        .columns(columns)
        .values(sea_values)?
        .returning(Query::returning().columns([CommonIden::Id]));

    // -- Exec query
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let sqlx_query = sqlx::query_as_with::<_, (i64,), _>(&sql, values);
    // NOTE: For now, we will use the _txn for all create.
    //       We could have a with_txn as function argument if perf is an issue (it should not be)
    let (id,) = mm.dbx().fetch_one(sqlx_query).await?;
    Ok(id)
}

pub async fn create_many<MC, E>(mm: &ModelManager, data: Vec<E>) -> Result<Vec<i64>>
where
    MC: DbBmc,
    E: HasSeaFields,
{
    let ctx = mm.ctx_ref()?;
    let mut ids = Vec::with_capacity(data.len());

    // Prepare insert query
    let mut query = Query::insert();

    for item in data {
        let mut fields = item.not_none_sea_fields();
        fields = prep_fields_for_create::<MC>(fields, ctx);
        let (columns, sea_values) = fields.for_sea_insert();

        // Append values for each item
        query.into_table(MC::table_ref()).columns(columns.clone()).values(sea_values)?;
    }

    query.returning(Query::returning().columns([CommonIden::Id]));

    // Execute query
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let sqlx_query = sqlx::query_as_with::<_, (i64,), _>(&sql, values);

    mm.dbx().begin_txn().await?;
    let rows = mm.dbx().fetch_all(sqlx_query).await?;
    for row in rows {
        let (id,): (i64,) = row;
        ids.push(id);
    }
    mm.dbx().commit_txn().await?;

    Ok(ids)
}

pub async fn insert<MC, E>(mm: &ModelManager, data: E) -> Result<()>
where
    MC: DbBmc,
    E: HasSeaFields,
{
    let ctx = mm.ctx_ref()?;

    // -- Extract fields (name / sea-query value expression)
    let mut fields = data.not_none_sea_fields();
    fields = prep_fields_for_create::<MC>(fields, ctx);

    // -- Build query
    let (columns, sea_values) = fields.for_sea_insert();
    let mut query = Query::insert();
    query.into_table(MC::table_ref()).columns(columns).values(sea_values)?;
    // .returning(Query::returning().columns([CommonIden::Id]));

    // -- Exec query
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let sqlx_query = sqlx::query_with(&sql, values);

    let count = mm.dbx().execute(sqlx_query).await?;
    if count == 0 {
        Ok(())
    } else {
        // TODO 需要更有效的插入失败错误
        Err(Error::CountFail)
    }
}

pub async fn insert_many<MC, E>(mm: &ModelManager, data: impl IntoIterator<Item = E>) -> Result<u64>
where
    MC: DbBmc,
    E: HasSeaFields,
{
    let ctx = mm.ctx_ref()?;

    // Prepare insert query
    let mut query = Query::insert();

    for item in data {
        let mut fields = item.not_none_sea_fields();
        fields = prep_fields_for_create::<MC>(fields, ctx);
        let (columns, sea_values) = fields.for_sea_insert();

        // Append values for each item
        query.into_table(MC::table_ref()).columns(columns).values(sea_values)?;
    }

    // Execute query
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let sqlx_query = sqlx::query_with(&sql, values);
    let rows = mm.dbx().execute(sqlx_query).await?;
    Ok(rows)
}

pub async fn get_by_id<MC, E>(mm: &ModelManager, id: Id) -> Result<E>
where
    MC: DbBmc,
    E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
    E: HasSeaFields,
{
    let filter: FilterGroups = id.to_filter_node("id").into();
    find::<MC, E, _>(mm, filter).await?.ok_or_else(|| Error::EntityNotFound {
        schema: MC::SCHEMA,
        entity: MC::TABLE,
        id,
    })
}

pub async fn find<MC, E, F>(mm: &ModelManager, filter: F) -> Result<Option<E>>
where
    MC: DbBmc,
    E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
    E: HasSeaFields,
    F: Into<FilterGroups>,
{
    // -- Build the query
    let mut query = Query::select();
    query.from(MC::table_ref()).columns(E::sea_column_refs());

    // condition from filter
    let filters: FilterGroups = filter.into();
    let cond: Condition = filters.try_into()?;
    query.cond_where(cond);

    // -- Execute the query
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let sqlx_query = sqlx::query_as_with::<_, E, _>(&sql, values);
    let entity = mm.dbx().fetch_optional(sqlx_query).await?;

    Ok(entity)
}

pub async fn list<MC, E, F>(mm: &ModelManager, filter: Option<F>, list_options: Option<ListOptions>) -> Result<Vec<E>>
where
    MC: DbBmc,
    F: Into<FilterGroups>,
    E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
    E: HasSeaFields,
{
    // -- Build the query
    let mut query = Query::select();
    query.from(MC::table_ref()).columns(E::sea_column_refs());

    // condition from filter
    if let Some(filter) = filter {
        let filters: FilterGroups = filter.into();
        let cond: Condition = filters.try_into()?;
        query.cond_where(cond);
    }
    // list options
    let list_options = compute_list_options::<MC>(list_options)?;
    list_options.apply_to_sea_query(&mut query);

    // -- Execute the query
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let sqlx_query = sqlx::query_as_with::<_, E, _>(&sql, values);
    let entities = mm.dbx().fetch_all(sqlx_query).await?;

    Ok(entities)
}

pub async fn count<MC, F>(mm: &ModelManager, filter: Option<F>) -> Result<i64>
where
    MC: DbBmc,
    F: Into<FilterGroups>,
{
    let db = mm.dbx().db();
    // -- Build the query
    let mut query = Query::select().from(MC::table_ref()).expr(Expr::col(sea_query::Asterisk).count()).to_owned();

    // condition from filter
    if let Some(filter) = filter {
        let filters: FilterGroups = filter.into();
        let cond: Condition = filters.try_into()?;
        query.cond_where(cond);
    }

    let query_str = query.to_string(PostgresQueryBuilder);

    let result = sqlx::query(&query_str).fetch_one(db).await.map_err(|_| Error::CountFail)?;

    let count: i64 = result.try_get("count").map_err(|_| Error::CountFail)?;

    Ok(count)
}

pub async fn update<MC, E>(mm: &ModelManager, id: Id, data: E) -> Result<()>
where
    MC: DbBmc,
    E: HasSeaFields,
{
    let ctx = mm.ctx_ref()?;

    // -- Prep Fields
    let mut fields = data.not_none_sea_fields();
    if MC::has_modification_timestamps() {
        fields = prep_fields_for_update::<MC>(fields, ctx);
    }

    // -- Build query
    let fields = fields.for_sea_update();
    let mut query = Query::update();
    query.table(MC::table_ref()).values(fields).and_where(Expr::col(CommonIden::Id).eq(id.clone()));

    // -- Execute query
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let sqlx_query = sqlx::query_with(&sql, values);
    let count = mm.dbx().execute(sqlx_query).await?;

    // -- Check result
    _check_result::<MC>(count, id)
}

pub async fn delete<MC>(mm: &ModelManager, id: Id) -> Result<()>
where
    MC: DbBmc,
{
    let ctx = mm.ctx_ref()?;

    // -- Build query
    let (sql, values) = if MC::use_logical_deletion() {
        // -- Prep Fields
        let mut fields = SeaFields::new(vec![SeaField::new(CommonIden::LogiscalDeletion, true)]);
        if MC::has_modification_timestamps() {
            fields = prep_fields_for_update::<MC>(fields, ctx);
        }

        let fields = fields.for_sea_update();
        Query::update()
            .table(MC::table_ref())
            .values(fields)
            .and_where(Expr::col(CommonIden::Id).eq(id.clone()))
            .build_sqlx(PostgresQueryBuilder)
    } else {
        Query::delete()
            .from_table(MC::table_ref())
            .and_where(Expr::col(CommonIden::Id).eq(id.clone()))
            .build_sqlx(PostgresQueryBuilder)
    };

    // -- Execute query
    // let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let sqlx_query = sqlx::query_with(&sql, values);
    let count = mm.dbx().execute(sqlx_query).await?;

    _check_result::<MC>(count, id)
}

/// Check result
fn _check_result<MC>(count: u64, id: Id) -> Result<()>
where
    MC: DbBmc,
{
    if count == 0 {
        Err(Error::EntityNotFound { schema: MC::SCHEMA, entity: MC::TABLE, id })
    } else {
        Ok(())
    }
}

pub async fn delete_many<MC>(mm: &ModelManager, ids: Vec<Id>) -> Result<u64>
where
    MC: DbBmc,
{
    if ids.is_empty() {
        return Ok(0);
    }

    let ids_len = ids.len();

    // -- Build query
    let mut query = Query::delete();
    query.from_table(MC::table_ref()).and_where(Expr::col(CommonIden::Id).is_in(ids));

    // -- Execute query
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let sqlx_query = sqlx::query_with(&sql, values);
    let n = mm.dbx().execute(sqlx_query).await?;

    // -- Check result
    check_number_of_affected::<MC>(ids_len, n)
}

pub fn compute_list_options<MC>(list_options: Option<ListOptions>) -> Result<ListOptions>
where
    MC: DbBmc,
{
    if let Some(mut list_options) = list_options {
        // Validate the limit.
        if let Some(limit) = list_options.limit {
            if limit > MC::LIST_LIMIT_MAX {
                return Err(Error::ListLimitOverMax { max: MC::LIST_LIMIT_MAX, actual: limit });
            }
        } else {
            // Set the default limit if no limit
            list_options.limit = Some(MC::LIST_LIMIT_DEFAULT);
        }
        Ok(list_options)
    }
    // When None, return default
    else {
        Ok(ListOptions { limit: Some(MC::LIST_LIMIT_DEFAULT), offset: None, order_bys: Some("id".into()) })
    }
}
