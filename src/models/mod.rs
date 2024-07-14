use crate::schema::posts;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Insertable, Serialize, Selectable, Queryable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub id: String,
    pub title: String,
    pub body: String,
}

#[derive(Deserialize)]
pub struct NewPostRequest {
    pub title: String,
    pub body: String,
}

#[derive(Serialize)]
pub struct NewPostResponse {
    pub uuid: Uuid,
}
