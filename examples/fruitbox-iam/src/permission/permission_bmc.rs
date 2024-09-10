use modql::{
  field::HasSeaFields,
  filter::{FilterGroups, ListOptions},
};
use sea_query::{Condition, Expr, PostgresQueryBuilder, Query, SelectStatement};
use sea_query_binder::SqlxBinder;
use ultimate_api::v1::{Page, PagePayload, Pagination};
use ultimate_db::{
  base::{self, compute_list_options, DbBmc},
  generate_common_bmc_fns, ModelManager, Result,
};

use crate::role::role_permission::{RolePermissionBmc, RolePermissionIden};

use super::{Permission, PermissionFilters, PermissionForCreate, PermissionForUpdate, PermissionIden};

pub struct PermissionBmc;
impl DbBmc for PermissionBmc {
  const SCHEMA: &'static str = "iam";
  const TABLE: &'static str = "permission";
}

generate_common_bmc_fns!(
  Bmc: PermissionBmc,
  Entity: Permission,
  ForCreate: PermissionForCreate,
  ForUpdate: PermissionForUpdate,
);

impl PermissionBmc {
  pub async fn page(
    mm: &ModelManager,
    filters: PermissionFilters,
    pagination: Pagination,
  ) -> Result<PagePayload<Permission>> {
    let total_size = Self::count(mm, filters.clone()).await?;
    let items = Self::find_many(mm, filters, Some((&pagination).into())).await?;
    Ok(PagePayload::new(Page::new(&pagination, total_size), items))
  }

  pub async fn count(mm: &ModelManager, filters: PermissionFilters) -> Result<i64> {
    let count = base::count_on::<Self, _>(mm, |query| make_select_statement(query, filters)).await?;
    Ok(count)
  }

  pub async fn find_many(
    mm: &ModelManager,
    filters: PermissionFilters,
    list_options: Option<ListOptions>,
  ) -> Result<Vec<Permission>> {
    // -- Build the query
    let mut query = Query::select();
    query.from(Self::table_ref());

    // select columns
    query.columns(Permission::sea_column_refs_with_rel(PermissionIden::Table));

    // condition from filter
    make_select_statement(&mut query, filters)?;

    // list options
    let list_options = compute_list_options::<Self>(list_options)?;
    list_options.apply_to_sea_query(&mut query);

    // -- Execute the query
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let sqlx_query = sqlx::query_as_with::<_, Permission, _>(&sql, values);
    let entities = mm.dbx().fetch_all(sqlx_query).await?;

    Ok(entities)
  }
}

fn make_select_statement(query: &mut SelectStatement, filter: PermissionFilters) -> Result<()> {
  // condition from filter
  let filters: FilterGroups = filter.filter.into();
  let cond: Condition = filters.try_into()?;
  query.cond_where(cond);
  query.and_where(Expr::col(PermissionIden::Id).in_subquery({
    let mut q = Query::select();
    q.from(RolePermissionBmc::table_ref()).column(RolePermissionIden::PermissionId);
    let sub_cond: Condition = filter.role_perm_filter.try_into()?;
    q.cond_where(sub_cond);
    q
  }));

  Ok(())
}
