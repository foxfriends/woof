use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LimitOffsetPagination<T> {
    pub total: usize,
    pub items: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageNumberPagination<T> {
    pub total: usize,
    pub items: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CursorPagination<T> {
    pub items: Vec<T>,
    pub cursor: String,
}
