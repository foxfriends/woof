mod error;
mod pagination;

mod rest_model;
mod rest_trait;

pub use error::{Error, Result};
pub use pagination::{CursorPagination, LimitOffsetPagination, PageNumberPagination};
pub use rest_model::RestModel;
pub use rest_trait::{Create, Filter, Rest, Update};
