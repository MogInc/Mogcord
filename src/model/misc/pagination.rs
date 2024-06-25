use axum::extract::Query;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination 
{
    pub page: usize,
    pub page_size: usize,
}

impl Pagination {
    const MIN_PAGE_NR: usize = 1;
    const MAX_PER_PAGE: usize = 50;

    fn new_valid(page: usize, per_page: usize) -> Self 
    {
        Self 
        {
            page: page.min(Self::MIN_PAGE_NR),
            page_size: per_page.min(Self::MAX_PER_PAGE),
        }
    }

    pub fn new(page : Option<Query<Pagination>>) -> Self
    {
        return page
        .map(|Query(pagination)| Self::new_valid(pagination.page, pagination.page_size))
        .unwrap_or_default();
    }
}

impl Default for Pagination 
{
    fn default() -> Self 
    {
        Self { page: 1, page_size: 25 }
    }
}