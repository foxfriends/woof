use sea_orm::{ActiveModelTrait, IntoActiveModel};
use serde::de::DeserializeOwned;

pub trait Update<A: ActiveModelTrait>: DeserializeOwned + Clone + IntoActiveModel<A> {}
