use super::PageNumberPagination;
use actix_web::{error, http::StatusCode, web, HttpResponse, Scope};
use sea_orm::{
    ActiveModelTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel, PaginatorTrait,
    PrimaryKeyTrait, QueryFilter,
};
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;

pub struct RestModel<Entity, ActiveModel, Insert, Update, Filter>(
    PhantomData<Entity>,
    PhantomData<ActiveModel>,
    PhantomData<Insert>,
    PhantomData<Update>,
    PhantomData<Filter>,
);

pub trait FilterSet {
    fn limit(&self) -> usize;
    fn offset(&self) -> usize;
    fn page(&self) -> usize;
    fn cursor(&self) -> Option<&str>;

    fn condition(&self) -> Condition;
}

impl<Entity, ActiveModel, Insert, Update, Filter>
    RestModel<Entity, ActiveModel, Insert, Update, Filter>
where
    Entity: EntityTrait + 'static,
    Entity::Model: Serialize + DeserializeOwned + IntoActiveModel<ActiveModel> + Sync + Send,
    Insert: Serialize + DeserializeOwned + IntoActiveModel<ActiveModel> + Clone + 'static,
    Update: Serialize + DeserializeOwned + IntoActiveModel<ActiveModel> + Clone + 'static,
    Filter: DeserializeOwned + FilterSet + 'static,
    ActiveModel: ActiveModelTrait<Entity = Entity> + 'static,
    <Entity::PrimaryKey as PrimaryKeyTrait>::ValueType: DeserializeOwned + Clone,
{
    pub fn service(path: &str) -> Scope {
        web::scope(path)
            .route("/", web::get().to(Self::list))
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

    async fn list(
        query: web::Query<Filter>,
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
