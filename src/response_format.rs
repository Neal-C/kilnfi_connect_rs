use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReturnedData<Type> {
    pub data: Type,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pagination {
    pub current_page: u64,
    pub next_page: u64,
    pub previous_page: u64,
    pub page_size: u64,
    pub total_pages: u64,
    pub total_entries: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaginatedData<Type> {
    pub data: Type,
    pub pagination: Pagination,
}
