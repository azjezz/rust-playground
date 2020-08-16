use actix_web::web;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tera::Context;
use uuid::Uuid;

use crate::repository::notes;
use crate::models::NewNote;
use crate::service::Templating;

pub async fn index(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
    templating: web::Data<Templating>
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let connection = pool.get().expect("couldn't get db connection from pool");
    let mut notes = web::block(move || notes::find_all(&connection))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            actix_web::HttpResponse::InternalServerError().finish()
        })?
        .unwrap();

    notes.reverse();

    let mut context = Context::new();
    context.insert("notes", &notes);

    return templating.render("index.html.twig", &context);
}

pub async fn new(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
    form: web::Form<NewNote>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let connection = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let _note = web::block(move || notes::insert(&form.text, &connection))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            actix_web::HttpResponse::InternalServerError().finish()
        })?;


    let response = actix_web::HttpResponse::Found().header("Location", "/").finish();

    Ok(response)
}

pub async fn delete(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
    note_uid: web::Path<Uuid>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let connection = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    web::block(move || notes::delete(&note_uid.into_inner(), &connection))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            actix_web::HttpResponse::InternalServerError().finish()
        })?;

    let response = actix_web::HttpResponse::Found().header("Location", "/").finish();

    Ok(response)
}