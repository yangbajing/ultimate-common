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

#[derive(Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Page {
    pub page: i64,
    pub page_size: i64,
    pub total_size: i64,
    pub total_page: i64,
}

impl Page {
    pub fn new(page: &Pagination, total_size: i64) -> Self {
        let total_page = if total_size == 0 { 0 } else { (total_size + page.page_size - 1) / page.page_size };
        Self { page: page.page, page_size: page.page_size, total_size, total_page }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Pagination {
    #[serde(default = "default_page")]
    #[cfg_attr(feature = "utoipa", schema(default = default_page))]
    pub page: i64,

    #[serde(default = "default_page_size")]
    #[cfg_attr(feature = "utoipa", schema(default = default_page_size))]
    pub page_size: i64,

    pub sort_bys: Vec<SortBy>,

    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SortBy {
    /// 要排序字段
    pub f: String,

    /// 排序方法
    #[serde(default)]
    pub b: SortByDirection,
}

impl From<SortBy> for OrderBy {
    fn from(value: SortBy) -> Self {
        match value.b {
            SortByDirection::asc => OrderBy::Asc(value.f),
            SortByDirection::desc => OrderBy::Desc(value.f),
        }
    }
}

impl From<&SortBy> for OrderBy {
    fn from(value: &SortBy) -> Self {
        match value.b {
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

// pub static DEFAULT_PAGE_INFO: LazyLock<PageInfo> = LazyLock::new(|| PageInfo::default());

impl Pagination {
    pub fn offset(&self) -> i64 {
        if let Some(offset) = self.offset {
            return offset;
        }
        if self.page < 2 {
            return 0;
        }
        self.page_size * (self.page - 1)
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: default_page(),
            page_size: default_page_size(),
            sort_bys: Default::default(),
            offset: Default::default(),
        }
    }
}

impl From<Pagination> for ListOptions {
    fn from(value: Pagination) -> Self {
        let offset = Some(value.offset());
        let list: Vec<OrderBy> = value.sort_bys.into_iter().map(Into::into).collect();
        let order_bys = if list.is_empty() { None } else { Some(OrderBys::new(list)) };
        ListOptions { limit: Some(value.page_size), offset, order_bys }
    }
}

impl From<&Pagination> for ListOptions {
    fn from(value: &Pagination) -> Self {
        let offset = Some(value.offset());
        let list: Vec<OrderBy> = value.sort_bys.iter().map(Into::into).collect();
        let order_bys = if list.is_empty() { None } else { Some(OrderBys::new(list)) };
        ListOptions { limit: Some(value.page_size), offset, order_bys }
    }
}

fn default_page() -> i64 {
    1
}

fn default_page_size() -> i64 {
    20
}
