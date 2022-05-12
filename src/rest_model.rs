use crate::pagination::PageNumberPagination;
use crate::{extractors, middleware};
use crate::{Filter, Rest};
use actix_web::{
    body::BoxBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    error,
    http::StatusCode,
    web, Error, HttpRequest, HttpResponse, Scope,
};
use sea_orm::{
    sea_query::IntoValueTuple, ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    Iterable, PaginatorTrait, PrimaryKeyToColumn, PrimaryKeyTrait, QueryFilter,
};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

pub struct RestModel<T> {
    _pd: PhantomData<T>,
    path: String,
}

impl<T> RestModel<T>
where
    T: Rest + 'static,
    <T::Entity as EntityTrait>::Model: IntoActiveModel<T::ActiveModel> + Send + Sync,
    <<T::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType:
        DeserializeOwned + Clone,
{
    pub fn new(path: impl AsRef<str>) -> Self {
        Self {
            _pd: PhantomData,
            path: path.as_ref().to_owned(),
        }
    }

    // TODO: is this the best way to write this return type?
    pub fn as_service(
        &self,
    ) -> Scope<
        impl ServiceFactory<
            ServiceRequest,
            Config = (),
            Response = ServiceResponse<BoxBody>,
            Error = Error,
            InitError = (),
        >,
    > {
        let id_path = T::id_path(None);
        web::scope(&self.path)
            .route("", web::get().to(Self::list))
            .route("/new", web::post().to(Self::create))
            .service(
                web::resource(&id_path)
                    .wrap(middleware::PrimaryKey::<T>::default())
                    .route(web::get().to(Self::get)),
            )
            .service(
                web::resource(&id_path)
                    .wrap(middleware::PrimaryKey::<T>::default())
                    .route(web::delete().to(Self::delete)),
            )
            .service(
                web::resource(&id_path)
                    .wrap(middleware::PrimaryKey::<T>::default())
                    .route(web::patch().to(Self::update)),
            )
            .service(
                web::resource(&id_path)
                    .wrap(middleware::PrimaryKey::<T>::default())
                    .route(web::put().to(Self::replace)),
            )
    }

    fn set_primary_key(
        primary_key: <<T::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType,
        active_model: &mut T::ActiveModel,
    ) {
        let pk_columns =
            <T::Entity as EntityTrait>::PrimaryKey::iter().map(PrimaryKeyToColumn::into_column);
        let pk_values = primary_key.into_value_tuple();
        for (column, value) in pk_columns.zip(pk_values) {
            active_model.set(column, value);
        }
    }

    async fn get(
        id: extractors::PrimaryKey<T>,
        db: web::Data<DatabaseConnection>,
    ) -> crate::Result<web::Json<T::Repr>> {
        T::Entity::find_by_id(id.clone())
            .one(&**db)
            .await?
            .map(From::from)
            .map(web::Json)
            .ok_or_else(|| error::ErrorNotFound("Not found").into())
    }

    async fn delete(
        id: extractors::PrimaryKey<T>,
        db: web::Data<DatabaseConnection>,
    ) -> crate::Result<HttpResponse> {
        T::Entity::delete_by_id(id.clone()).exec(&**db).await?;
        Ok(HttpResponse::new(StatusCode::NO_CONTENT))
    }

    async fn create(
        body: web::Json<T::Create>,
        db: web::Data<DatabaseConnection>,
    ) -> crate::Result<web::Json<T::Repr>> {
        Ok(web::Json(
            T::Entity::insert(body.clone().into_active_model())
                .exec_with_returning(&**db)
                .await?
                .into(),
        ))
    }

    async fn update(
        id: extractors::PrimaryKey<T>,
        body: web::Json<T::Update>,
        db: web::Data<DatabaseConnection>,
    ) -> crate::Result<web::Json<T::Repr>> {
        let mut active_model = body.clone().into_active_model();
        Self::set_primary_key(id.clone(), &mut active_model);
        Ok(web::Json(
            T::Entity::update(active_model).exec(&**db).await?.into(),
        ))
    }

    async fn replace(
        request: HttpRequest,
        body: web::Json<T::Create>,
        db: web::Data<DatabaseConnection>,
    ) -> crate::Result<web::Json<T::Repr>> {
        let mut active_model = body.clone().into_active_model();
        let id = T::id_from_path(None, request.match_info())?;
        Self::set_primary_key(id, &mut active_model);
        Ok(web::Json(
            T::Entity::insert(active_model)
                .exec_with_returning(&**db)
                .await?
                .into(),
        ))
    }

    async fn list(
        query: web::Query<T::Filter>,
        db: web::Data<DatabaseConnection>,
    ) -> crate::Result<web::Json<PageNumberPagination<T::Repr>>> {
        let page = query.page();
        let limit = query.limit();
        let pagination = T::Entity::find()
            .filter(query.condition())
            .paginate(&**db, limit);
        let total = pagination.num_items().await?;
        let items = pagination
            .fetch_page(page)
            .await?
            .into_iter()
            .map(From::from)
            .collect();
        Ok(web::Json(PageNumberPagination { total, items }))
    }
}
