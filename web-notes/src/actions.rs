use actix_web::{get, post, web};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::r2d2::PooledConnection;
use tera::Context;
use uuid::Uuid;

use crate::models::NewNote;
use crate::models::Note;
use crate::service::Templating;

#[get("/")]
pub async fn index(templating: web::Data<Templating>) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let mut context = Context::new();
    context.insert("action", "/new");

    return templating.render("index.html.twig", &context);
}

#[post("/new")]
pub async fn new(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
    form: web::Form<NewNote>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let connection = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let _note = web::block(move || insert_note(&form.text, &connection))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            actix_web::HttpResponse::InternalServerError().finish()
        })?;

    let response = actix_web::HttpResponse::TemporaryRedirect().header("Location", "/").finish();

    Ok(response)
}

fn insert_note(note: &str, connection: &PooledConnection<ConnectionManager<SqliteConnection>>) -> Result<Note, diesel::result::Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::notes::dsl::*;

    let new_note = Note {
        id: Uuid::new_v4().to_string(),
        text: note.parse().unwrap(),
    };

    diesel::insert_into(notes).values(&new_note).execute(&*connection).expect("Unable to insert new note.");

    Ok(new_note)
}