use actix_web::dev::{Path, Url};
use sea_orm::{
    ActiveModelTrait, Condition, EntityTrait, IdenStatic, IntoActiveModel, Iterable,
    PrimaryKeyToColumn, PrimaryKeyTrait,
};
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

pub trait Rest
where
    <Self::Entity as EntityTrait>::Model: IntoActiveModel<Self::ActiveModel> + Send + Sync,
{
    type Entity: EntityTrait;
    type Repr: Serialize + From<<Self::Entity as EntityTrait>::Model>;
    type ActiveModel: ActiveModelTrait<Entity = Self::Entity>;
    type Filter: Filter;
    type Create: Create<Self::ActiveModel>;
    type Update: Update<Self::ActiveModel>;

    fn id_from_path(
        scope: Option<&str>,
        path: &Path<Url>,
    ) -> crate::Result<<<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>;

    fn id_path(scope: Option<&str>) -> String {
        let scope = scope.map(|scope| format!("{scope}_")).unwrap_or_default();
        <Self::Entity as EntityTrait>::PrimaryKey::iter()
            .map(|key_col| format!("{{{}}}", scope.clone() + key_col.into_column().as_str()))
            .collect::<Vec<_>>()
            .join("/")
    }
}
