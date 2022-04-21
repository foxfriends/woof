use crate::traits::Rest;
use sea_orm::entity::{EntityTrait, PrimaryKeyTrait};

pub(crate) struct PrimaryKeyExtension<T>(
    pub <<T::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType,
)
where
    T: Rest;
