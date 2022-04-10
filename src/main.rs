use actix_web::{web, App, HttpServer};
use sea_orm::Database;

mod entity;
mod error;
mod rest_model;

pub use error::{Error, Result};
pub use rest_model::RestModel;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let db = Database::connect(std::env::var("DATABASE_URL").unwrap()).await?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(RestModel::<
                entity::users::Entity,
                entity::users::ActiveModel,
                entity::users::CreateModel,
                entity::users::UpdateModel,
            >::service("/users"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
