use actix_web::{error, http::StatusCode, web, HttpResponse, Scope};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, PrimaryKeyTrait,
};
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;

pub struct RestModel<Entity, ActiveModel, Insert, Update>(
    PhantomData<Entity>,
    PhantomData<ActiveModel>,
    PhantomData<Insert>,
    PhantomData<Update>,
);

impl<Entity, ActiveModel, Insert, Update> RestModel<Entity, ActiveModel, Insert, Update>
where
    Entity: EntityTrait + 'static,
    Entity::Model: Serialize + DeserializeOwned + IntoActiveModel<ActiveModel>,
    Insert: Serialize + DeserializeOwned + IntoActiveModel<ActiveModel> + Clone + 'static,
    Update: Serialize + DeserializeOwned + IntoActiveModel<ActiveModel> + Clone + 'static,
    ActiveModel: ActiveModelTrait<Entity = Entity> + 'static,
    <Entity::PrimaryKey as PrimaryKeyTrait>::ValueType: DeserializeOwned + Clone,
{
    pub fn service(path: &str) -> Scope {
        web::scope(path)
            .route("/new", web::post().to(Self::create))
            .route("/{id}", web::get().to(Self::get))
            .route("/{id}", web::delete().to(Self::delete))
            .route("/{id}", web::patch().to(Self::update))
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
        body: web::Json<Insert>,
        db: web::Data<DatabaseConnection>,
    ) -> crate::Result<web::Json<Entity::Model>> {
        Ok(web::Json(
            Entity::insert(body.clone().into_active_model())
                .exec_with_returning(&**db)
                .await?,
        ))
    }

    async fn update(
        body: web::Json<Update>,
        db: web::Data<DatabaseConnection>,
    ) -> crate::Result<web::Json<Entity::Model>> {
        Ok(web::Json(
            Entity::insert(body.clone().into_active_model())
                .exec_with_returning(&**db)
                .await?,
        ))
    }
}
