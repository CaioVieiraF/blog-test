use crate::schema::posts;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Post struct that matches exactally the DB table
#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Post {
    pub id: String,
    pub title: String,
    pub body: String,
    pub public: bool,
    pub draft: bool,
}

// Struct that matches the post creation on the database
#[derive(Insertable, Serialize, Selectable, Queryable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub id: String,
    pub title: String,
    pub body: String,
}

// Struct that matches the post creation request
#[derive(Deserialize)]
pub struct NewPostRequest {
    pub title: String,
    pub body: String,
}

// Struct that matches the post creation response
#[derive(Serialize)]
pub struct NewPostResponse {
    pub uuid: Uuid,
}
