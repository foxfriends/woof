use super::{Create, Filter, Update};
use actix_web::dev::{Path, Url};
use sea_orm::{
    ActiveModelTrait, EntityTrait, IdenStatic, Iterable, PrimaryKeyToColumn, PrimaryKeyTrait,
};
use serde::Serialize;

pub trait Rest {
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
