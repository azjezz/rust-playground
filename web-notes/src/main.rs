//! Actix web Diesel integration example
//!
//! Diesel does not support tokio, so we have to run it in separate threads using the web::block
//! function which offloads blocking code (like Diesel's) in order to not block the server's thread.

#[macro_use]
extern crate diesel;

use actix_files::Files;
use actix_web::{App, HttpServer, web, middleware};
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::SqliteConnection;

mod service;
mod repository;
mod actions;
mod models;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let bind = "127.0.0.1:8080";

    println!("Starting server at: {}", &bind);

    HttpServer::new(|| {
        // set up database connection pool
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let templating = service::Templating::new("templates", true);

        return App::new()
            .data(templating)
            .data(pool.clone())
            .wrap(middleware::Logger::default()) // enable logger
            .service(web::resource("/")             .name("index")  .route(web::get()   .to(actions::index)))
            .service(web::resource("/new")          .name("new")    .route(web::post()  .to(actions::new)))
            .service(web::resource("/delete/{uid}") .name("delete") .route(web::post()  .to(actions::delete)))
            .service(
                Files::new("/static", concat!(env!("CARGO_MANIFEST_DIR"), "/static/")).show_files_listing()
            );
    }).bind(bind)?.run().await
}
