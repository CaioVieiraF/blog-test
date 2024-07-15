use diesel::{associations::Identifiable, deserialize::Queryable, prelude::Insertable, Selectable};
use serde::{Deserialize, Serialize};

use super::post::Post;

// User struct that matches exactally the DB table
#[derive(Queryable, Selectable, Serialize, Insertable, Deserialize, Identifiable, PartialEq)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
}

// Struct that matches the user creation request
#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UniqueUserInfo {
    pub name: String,
    pub email: String,
    pub posts: Vec<Post>,
}
