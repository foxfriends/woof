use super::PageNumberPagination;
use crate::{Filter, Rest};
use actix_web::{error, http::StatusCode, web, HttpRequest, HttpResponse, Scope};
use sea_orm::{
    sea_query::IntoValueTuple, ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    Iterable, PaginatorTrait, PrimaryKeyToColumn, PrimaryKeyTrait, QueryFilter,
};
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;

pub struct RestModel<Entity: Rest>
where
    Entity::Model:
        Serialize + DeserializeOwned + IntoActiveModel<Entity::ActiveModel> + Send + Sync,
{
    _pd: PhantomData<Entity>,
    scope: Scope,
}

impl<Entity: Rest> RestModel<Entity>
where
    Entity::Model:
        Serialize + DeserializeOwned + IntoActiveModel<Entity::ActiveModel> + Send + Sync,
    <Entity::PrimaryKey as PrimaryKeyTrait>::ValueType: DeserializeOwned + Clone,
{
    pub fn new(path: &str) -> Self {
        let id_path = Entity::id_path(None);
        let scope = web::scope(path)
            .route("", web::get().to(Self::list))
            .route("/new", web::post().to(Self::create))
            .route(&id_path, web::get().to(Self::get))
            .route(&id_path, web::delete().to(Self::delete))
            .route(&id_path, web::patch().to(Self::update))
            .route(&id_path, web::put().to(Self::replace));
        Self {
            _pd: PhantomData,
            scope,
        }
    }

    fn set_primary_key(
        primary_key: <Entity::PrimaryKey as PrimaryKeyTrait>::ValueType,
        active_model: &mut Entity::ActiveModel,
    ) {
        let pk_columns =
            <Entity as EntityTrait>::PrimaryKey::iter().map(PrimaryKeyToColumn::into_column);
        let pk_values = primary_key.into_value_tuple();
        for (column, value) in pk_columns.zip(pk_values) {
            active_model.set(column, value);
        }
    }

    pub fn instance_service<F>(self, action: F) -> Self
    where
        F: actix_web::dev::HttpServiceFactory + 'static,
    {
        let id_path = Entity::id_path(None);
        let scope = self
            .scope
            .service(web::scope(&id_path).service(action));
        Self { scope, ..self }
    }

    pub fn into_service(self) -> Scope {
        self.scope
    }

    async fn get(
        request: HttpRequest,
        db: web::Data<DatabaseConnection>,
    ) -> crate::Result<web::Json<Entity::Model>> {
        let id = Entity::id_from_path(None, request.match_info())?;
        Entity::find_by_id(id)
            .one(&**db)
            .await?
            .map(web::Json)
            .ok_or_else(|| error::ErrorNotFound("Not found").into())
    }

    async fn delete(
        request: HttpRequest,
        db: web::Data<DatabaseConnection>,
    ) -> crate::Result<HttpResponse> {
        let id = Entity::id_from_path(None, request.match_info())?;
        Entity::delete_by_id(id).exec(&**db).await?;
        Ok(HttpResponse::new(StatusCode::NO_CONTENT))
    }

    async fn create(
        body: web::Json<Entity::Create>,
        db: web::Data<DatabaseConnection>,
    ) -> crate::Result<web::Json<Entity::Model>> {
        Ok(web::Json(
            Entity::insert(body.clone().into_active_model())
                .exec_with_returning(&**db)
                .await?,
        ))
    }

    async fn update(
        request: HttpRequest,
        body: web::Json<Entity::Update>,
        db: web::Data<DatabaseConnection>,
    ) -> crate::Result<web::Json<Entity::Model>> {
        let mut active_model = body.clone().into_active_model();
        let id = Entity::id_from_path(None, request.match_info())?;
        Self::set_primary_key(id, &mut active_model);
        Ok(web::Json(Entity::update(active_model).exec(&**db).await?))
    }

    async fn replace(
        request: HttpRequest,
        body: web::Json<Entity::Create>,
        db: web::Data<DatabaseConnection>,
    ) -> crate::Result<web::Json<Entity::Model>> {
        let mut active_model = body.clone().into_active_model();
        let id = Entity::id_from_path(None, request.match_info())?;
        Self::set_primary_key(id, &mut active_model);
        Ok(web::Json(
            Entity::insert(active_model)
                .exec_with_returning(&**db)
                .await?,
        ))
    }

    async fn list(
        query: web::Query<Entity::Filter>,
        db: web::Data<DatabaseConnection>,
    ) -> crate::Result<web::Json<PageNumberPagination<Entity::Model>>> {
        let page = query.page();
        let limit = query.limit();
        let pagination = Entity::find()
            .filter(query.condition())
            .paginate(&**db, limit);
        let total = pagination.num_items().await?;
        let items = pagination.fetch_page(page).await?;
        Ok(web::Json(PageNumberPagination { total, items }))
    }
}
