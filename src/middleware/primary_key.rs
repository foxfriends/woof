use crate::extensions::PrimaryKeyExtension;
use crate::traits::Rest;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use std::future::{ready, Ready};
use std::marker::PhantomData;

pub struct PrimaryKey<T>(PhantomData<T>);

impl<T> Default for PrimaryKey<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<S, B, T> Transform<S, ServiceRequest> for PrimaryKey<T>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
    T: Rest + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = PrimaryKeyMiddleware<T, S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PrimaryKeyMiddleware {
            service,
            _pd: PhantomData,
        }))
    }
}

pub struct PrimaryKeyMiddleware<T, S> {
    service: S,
    _pd: PhantomData<T>,
}

impl<S, B, T> Service<ServiceRequest> for PrimaryKeyMiddleware<T, S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
    T: Rest + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = S::Future;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if let Some(pk) = T::id_from_path(None, req.match_info()).ok() {
            req.extensions_mut().insert(PrimaryKeyExtension::<T>(pk));
        }

        self.service.call(req)
    }
}
