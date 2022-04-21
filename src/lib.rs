pub mod error;
mod pagination;

mod rest_model;
mod traits;

pub use error::{Error, Result};
pub use pagination::{CursorPagination, LimitOffsetPagination, PageNumberPagination};
pub use rest_model::RestModel;
pub use traits::{Create, Filter, Rest, Update};
