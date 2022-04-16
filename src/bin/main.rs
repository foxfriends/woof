use actix_web::{web, App, HttpServer};
use sea_orm::Database;

use woof::RestModel;

mod entity;
mod model;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let db = Database::connect(std::env::var("DATABASE_URL").unwrap()).await?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(RestModel::<entity::prelude::Users>::service("/users"))
            .service(RestModel::<entity::prelude::Posts>::service("/posts"))
            .service(RestModel::<entity::prelude::Comments>::service("/comments"))
            .service(RestModel::<entity::prelude::Votes>::service("/votes"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
