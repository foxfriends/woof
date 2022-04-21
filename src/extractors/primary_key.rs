use crate::extensions::PrimaryKeyExtension;
use crate::traits::Rest;
use actix_web::{dev::Payload, error, Error, FromRequest, HttpMessage, HttpRequest};
use sea_orm::entity::{EntityTrait, PrimaryKeyTrait};
use std::future::{ready, Ready};
use std::ops::Deref;

pub struct PrimaryKey<T>(<<T::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType)
where
    T: Rest;

impl<T> FromRequest for PrimaryKey<T>
where
    T: Rest + 'static,
    <<T::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: Clone,
{
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(
            req.extensions()
                .get::<PrimaryKeyExtension<T>>()
                .ok_or_else(|| {
                    error::ErrorBadRequest("Expected PrimaryKey not found in request extensions")
                })
                .map(|value| Self(value.0.clone())),
        )
    }
}

impl<T> Deref for PrimaryKey<T>
where
    T: Rest,
{
    type Target = <<T::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
