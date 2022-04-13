use super::PageNumberPagination;
use actix_web::{error, http::StatusCode, web, HttpResponse, Scope};
use sea_orm::{
    ActiveModelTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel, PaginatorTrait,
    PrimaryKeyTrait, QueryFilter, PrimaryKeyToColumn, Iterable, sea_query::IntoValueTuple,
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
    fn set_primary_key(primary_key: <Entity::PrimaryKey as PrimaryKeyTrait>::ValueType, active_model: &mut ActiveModel) {
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
        path: web::Path<<Entity::PrimaryKey as PrimaryKeyTrait>::ValueType>,
        body: web::Json<Update>,
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
        body: web::Json<Insert>,
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
