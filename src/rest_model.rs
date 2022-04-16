use super::PageNumberPagination;
use serde::{Serialize, de::DeserializeOwned};
use actix_web::{error, http::StatusCode, web, HttpResponse, Scope};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, PaginatorTrait,
    PrimaryKeyTrait, QueryFilter, PrimaryKeyToColumn, Iterable, sea_query::IntoValueTuple,
};
use std::marker::PhantomData;
use crate::{Rest, Filter};

pub struct RestModel<Entity: Rest>(PhantomData<Entity>)
where
    Entity::Model: Serialize + DeserializeOwned + IntoActiveModel<Entity::ActiveModel> + Send + Sync,
    <Entity::PrimaryKey as PrimaryKeyTrait>::ValueType: DeserializeOwned + Clone,
;

impl<Entity: Rest> RestModel<Entity>
where
    Entity::Model: Serialize + DeserializeOwned + IntoActiveModel<Entity::ActiveModel> + Send + Sync,
    <Entity::PrimaryKey as PrimaryKeyTrait>::ValueType: DeserializeOwned + Clone,
{
    fn set_primary_key(primary_key: <Entity::PrimaryKey as PrimaryKeyTrait>::ValueType, active_model: &mut Entity::ActiveModel) {
        let pk_columns = <Entity as EntityTrait>::PrimaryKey::iter().map(PrimaryKeyToColumn::into_column);
        let pk_values = primary_key.into_value_tuple();
        for (column, value) in pk_columns.zip(pk_values) {
            active_model.set(column, value);
        }
    }

    pub fn service(path: &str) -> Scope {
        web::scope(path)
            .route("/", web::get().to(Self::list))
            .route("/new", web::post().to(Self::create))
            .route("/{id}", web::get().to(Self::get))
            .route("/{id}", web::delete().to(Self::delete))
            .route("/{id}", web::patch().to(Self::update))
            .route("/{id}", web::put().to(Self::replace))
    }

    async fn get(
        path: web::Path<<Entity::PrimaryKey as PrimaryKeyTrait>::ValueType>,
        db: web::Data<DatabaseConnection>,
    ) -> crate::Result<web::Json<Entity::Model>> {
        Entity::find_by_id(path.clone())
            .one(&**db)
            .await?
            .map(web::Json)
            .ok_or_else(|| error::ErrorNotFound("Not found").into())
    }

    async fn delete(
        path: web::Path<<Entity::PrimaryKey as PrimaryKeyTrait>::ValueType>,
        db: web::Data<DatabaseConnection>,
    ) -> crate::Result<HttpResponse> {
        Entity::delete_by_id(path.clone()).exec(&**db).await?;
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
        path: web::Path<<Entity::PrimaryKey as PrimaryKeyTrait>::ValueType>,
        body: web::Json<Entity::Update>,
        db: web::Data<DatabaseConnection>,
    ) -> crate::Result<web::Json<Entity::Model>> {
        let mut active_model = body.clone().into_active_model();
        Self::set_primary_key(path.clone(), &mut active_model);
        Ok(web::Json(
            Entity::update(active_model)
                .exec(&**db)
                .await?,
        ))
    }

    async fn replace(
        path: web::Path<<Entity::PrimaryKey as PrimaryKeyTrait>::ValueType>,
        body: web::Json<Entity::Create>,
        db: web::Data<DatabaseConnection>,
    ) -> crate::Result<web::Json<Entity::Model>> {
        let mut active_model = body.clone().into_active_model();
        Self::set_primary_key(path.clone(), &mut active_model);
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
