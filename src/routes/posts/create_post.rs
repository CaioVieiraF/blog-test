use actix_tests::DieselDB;
use actix_web::{post, web::Json, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use crate::models::{
    post::{NewPost, NewPostRequest},
    CreationResponse,
};

#[post("")]
async fn create_post<'a>(new_post: Json<NewPostRequest>) -> HttpResponse {
    use crate::schema::posts;

    log::debug!("Creating post");

    // Generate UUID to use as id for the post
    let post_id = Uuid::new_v4();

    // Opens the db connection
    let db_connection = &mut DieselDB::database_connection();
    let new_post = NewPost {
        id: post_id.into(),
        title: new_post.title.clone(),
        body: new_post.body.clone(),
        user_id: new_post.user_id.to_owned(),
    };

    // Create the post on DB
    let result = diesel::insert_into(posts::table)
        .values(new_post)
        .execute(db_connection);

    // If the creation fails, then log the error
    // and return an error. Otherwise return the
    // post id
    match result {
        Ok(_) => {
            log::info!("Post with id {post_id} created.");
            HttpResponse::Created().json(CreationResponse { uuid: post_id })
        }

        // Return Internal Server Error if the operation
        // did not work
        Err(e) => {
            log::error!("Error creating the post: {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
