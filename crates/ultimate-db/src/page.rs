use modql::filter::{ListOptions, OrderBy, OrderBys};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct PagePayload<T> {
  pub page: Page,
  pub records: Vec<T>,
}

impl<T> PagePayload<T> {
  pub fn new(page: Page, records: Vec<T>) -> Self {
    Self { page, records }
  }
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Page {
  pub page: i64,
  pub page_size: i64,
  pub total_size: i64,
  pub total_page: i64,
}

impl Page {
  pub fn new(pagination: &Pagination, total_size: i64) -> Self {
    let page = pagination.page();
    let page_size = pagination.page_size();
    let total_page = if total_size == 0 { 0 } else { (total_size + page_size - 1) / page_size };
    Self { page, page_size, total_size, total_page }
  }
}

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Pagination {
  pub page: Option<i64>,

  pub page_size: Option<i64>,

  pub sort_bys: Option<Vec<SortBy>>,

  pub offset: Option<i64>,
}

impl Pagination {
  pub fn page(&self) -> i64 {
    self.page.unwrap_or(default_page())
  }

  pub fn page_size(&self) -> i64 {
    self.page_size.unwrap_or(default_page_size())
  }

  pub fn sort_bys(&self) -> Vec<&SortBy> {
    match self.sort_bys.as_ref() {
      Some(vs) => vs.iter().collect(),
      None => vec![],
    }
  }

  pub fn offset(&self) -> i64 {
    if let Some(offset) = self.offset {
      return offset;
    }
    let page = self.page();
    let page_size = self.page_size();
    if page < 2 {
      return 0;
    }
    page_size * (page - 1)
  }
}

impl Default for Pagination {
  fn default() -> Self {
    Self {
      page: Some(default_page()),
      page_size: Some(default_page_size()),
      sort_bys: Default::default(),
      offset: Default::default(),
    }
  }
}

#[derive(Debug, Clone, Default, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SortBy {
  /// 要排序字段
  pub f: String,

  /// 排序方法
  #[serde(default)]
  pub d: SortByDirection,
}

impl From<SortBy> for OrderBy {
  fn from(value: SortBy) -> Self {
    match value.d {
      SortByDirection::asc => OrderBy::Asc(value.f),
      SortByDirection::desc => OrderBy::Desc(value.f),
    }
  }
}

impl From<&SortBy> for OrderBy {
  fn from(value: &SortBy) -> Self {
    match value.d {
      SortByDirection::asc => OrderBy::Asc(value.f.clone()),
      SortByDirection::desc => OrderBy::Desc(value.f.clone()),
    }
  }
}

#[derive(Debug, Clone, Default, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[allow(non_camel_case_types)]
pub enum SortByDirection {
  #[default]
  asc,
  desc,
}

impl From<Pagination> for ListOptions {
  fn from(value: Pagination) -> Self {
    let offset = Some(value.offset());
    let limit = Some(value.page_size.unwrap_or(default_page_size()));
    let order_bys = value.sort_bys.map(|v| OrderBys::new(v.into_iter().map(Into::into).collect()));

    ListOptions { limit, offset, order_bys }
  }
}

impl From<&Pagination> for ListOptions {
  fn from(value: &Pagination) -> Self {
    let offset = Some(value.offset());
    let limit = Some(value.page_size.unwrap_or(default_page_size()));
    let order_bys = value.sort_bys.as_ref().map(|v| OrderBys::new(v.iter().map(Into::into).collect()));
    ListOptions { limit, offset, order_bys }
  }
}

fn default_page() -> i64 {
  1
}

fn default_page_size() -> i64 {
  20
}
