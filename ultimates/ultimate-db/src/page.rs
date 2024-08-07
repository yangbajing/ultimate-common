use modql::filter::{FilterNode, ListOptions, OrderBys};
use serde::{Deserialize, Serialize};

pub trait ForPage {
    fn page(&self) -> &Pagination;

    fn filters(&self) -> &[FilterNode];

    fn get_list_options(&self) -> ListOptions {
        self.page().into()
    }
}

pub struct PagePayload<T>
where
    T: Serialize,
{
    pub page: Page,
    pub records: Vec<T>,
}

impl<T> PagePayload<T>
where
    T: Serialize,
{
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
    pub page: i64,

    #[serde(default = "default_page_size")]
    pub page_size: i64,

    pub order_bys: Option<OrderBys>,

    pub offset: Option<i64>,
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
        Self { page: default_page(), page_size: default_page_size(), order_bys: Default::default(), offset: None }
    }
}

impl From<Pagination> for ListOptions {
    fn from(value: Pagination) -> Self {
        ListOptions { limit: Some(value.page_size), offset: Some(value.offset()), order_bys: value.order_bys }
    }
}

impl From<&Pagination> for ListOptions {
    fn from(value: &Pagination) -> Self {
        ListOptions { limit: Some(value.page_size), offset: Some(value.offset()), order_bys: value.order_bys.clone() }
    }
}

fn default_page() -> i64 {
    1
}

fn default_page_size() -> i64 {
    20
}
