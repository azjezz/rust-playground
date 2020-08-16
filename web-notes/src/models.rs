use serde::{Deserialize, Serialize};

use crate::schema::notes;

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
pub struct Note {
    pub id: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewNote {
    pub text: String,
}