use actix_tests::DieselDB;
use actix_web::{post, web::Json, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use crate::models::{NewPost, NewPostRequest, NewPostResponse};

#[post("")]
async fn create_post<'a>(new_post: Json<NewPostRequest>) -> HttpResponse {
    use crate::schema::posts;

    log::debug!("Creating post");

    let post_id = Uuid::new_v4();
    let db_connection = &mut DieselDB::database_connection();
    let new_post = NewPost {
        id: post_id.into(),
        title: new_post.title.clone(),
        body: new_post.body.to_owned(),
    };

    diesel::insert_into(posts::table)
        .values(new_post)
        .execute(db_connection)
        .expect("Error creating post");

    HttpResponse::Created().json(NewPostResponse { uuid: post_id })
}
