use modql::field::HasSeaFields;
use sqlx::{postgres::PgRow, FromRow};

#[allow(unused)]
pub trait DbRowType: HasSeaFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}
