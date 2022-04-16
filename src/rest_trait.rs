use sea_orm::{ActiveModelTrait, Condition, EntityTrait, IntoActiveModel};
use serde::{de::DeserializeOwned, Serialize};

pub trait Filter: DeserializeOwned {
    fn limit(&self) -> usize;
    fn offset(&self) -> usize;
    fn page(&self) -> usize;
    fn cursor(&self) -> Option<&str>;
    fn condition(&self) -> Condition;
}

pub trait Create<A: ActiveModelTrait>:
    Serialize + DeserializeOwned + Clone + IntoActiveModel<A>
{
}

pub trait Update<A: ActiveModelTrait>:
    Serialize + DeserializeOwned + Clone + IntoActiveModel<A>
{
}

pub trait Rest: EntityTrait {
    type ActiveModel: ActiveModelTrait<Entity = Self>;
    type Filter: Filter;
    type Create: Create<Self::ActiveModel>;
    type Update: Update<Self::ActiveModel>;
}
