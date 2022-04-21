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
            .service(
                RestModel::<model::users::RestModel>::as_service("/users"), // .service(RestModel::<model::posts::RestModel>::as_service("/posts"))
            )
            .service(RestModel::<model::posts::RestModel>::as_service("/posts"))
            .service(RestModel::<model::comments::RestModel>::as_service(
                "/comments",
            ))
            .service(RestModel::<model::votes::RestModel>::as_service("/votes"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
