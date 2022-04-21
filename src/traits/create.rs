use sea_orm::{ActiveModelTrait, IntoActiveModel};
use serde::de::DeserializeOwned;

pub trait Create<A: ActiveModelTrait>: DeserializeOwned + Clone + IntoActiveModel<A> {}
