use sea_orm::Condition;
use serde::de::DeserializeOwned;

pub trait Filter: DeserializeOwned {
    fn limit(&self) -> usize;
    fn offset(&self) -> usize;
    fn page(&self) -> usize;
    fn cursor(&self) -> Option<&str>;
    fn condition(&self) -> Condition;
}
