use super::user::User;
use crate::schema::posts;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// Post struct that matches exactally the DB table
#[derive(Queryable, Selectable, Serialize, Deserialize, Associations, Identifiable, PartialEq)]
#[diesel(belongs_to(User))]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Post {
    pub id: String,
    pub title: String,
    pub body: String,
    pub public: bool,
    pub draft: bool,
    pub user_id: String,
}

// Struct that matches the post creation on the database
#[derive(Insertable, Serialize, Selectable, Queryable, PartialEq)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub id: String,
    pub title: String,
    pub body: String,
    pub user_id: String,
}

// Struct that matches the post creation request
#[derive(Deserialize)]
pub struct NewPostRequest {
    pub title: String,
    pub body: String,
    pub user_id: String,
}
