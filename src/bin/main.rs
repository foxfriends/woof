use actix_web::{web, App, HttpServer};
use sea_orm::Database;

use woof::RestModel;

mod entity;
mod model;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    pretty_env_logger::init();
    let db = Database::connect(std::env::var("DATABASE_URL").unwrap()).await?;

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::NormalizePath::trim())
            .app_data(web::Data::new(db.clone()))
            .service(RestModel::<entity::prelude::Users>::new("/users").into_service())
            .service(RestModel::<entity::prelude::Posts>::new("/posts").into_service())
            .service(RestModel::<entity::prelude::Comments>::new("/comments").into_service())
            .service(RestModel::<entity::prelude::Votes>::new("/votes").into_service())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
