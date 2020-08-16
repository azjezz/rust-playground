use std::vec::Vec;

use diesel::prelude::*;
use uuid::Uuid;

use crate::models::Note;

pub fn find_all(connection: &SqliteConnection) -> Result<Option<Vec<Note>>, diesel::result::Error> {
    use crate::schema::notes::dsl::*;

    let note = notes
        .load(connection)
        .optional()?;

    Ok(note)
}

pub fn delete(uid: &Uuid, connection: &SqliteConnection) -> QueryResult<usize> {
    use crate::schema::notes::dsl::*;

    return diesel::delete(notes.filter(id.eq(uid.to_string()))).execute(&*connection);
}

pub fn insert(note: &str, connection: &SqliteConnection) -> Result<Note, diesel::result::Error> {
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