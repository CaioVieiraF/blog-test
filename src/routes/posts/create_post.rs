use crate::prelude::*;
use actix_tests::DieselDB;
use actix_web::{post, web::Json, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use crate::models::{
    post::{NewPost, NewPostRequest},
    CreationResponse,
};

#[post("")]
async fn create_post(new_post_request: Json<NewPostRequest>) -> HttpResponse {
    log::info!("Creating post");

    // Create the post object
    let new_post = new_post(new_post_request.into_inner());

    // register the post on the DB
    let result = register_post_on_db(new_post);

    // If the creation fails, then log the error
    // and return an error. Otherwise return the
    // post id
    match result {
        Ok(post_id) => {
            log::info!("Post with id {} created.", post_id);
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

fn new_post(post_data: NewPostRequest) -> NewPost {
    log::debug!("Creating post object");

    // Generate UUID to use as id for the post
    let post_id = Uuid::new_v4();

    // Returns the post object
    NewPost {
        id: post_id.into(),
        title: post_data.title.clone(),
        body: post_data.body.clone(),
        user_id: post_data.user_id.to_owned(),
    }
}

fn register_post_on_db(post: NewPost) -> Result<String> {
    log::debug!("Register new post on DB");
    use crate::schema::posts;

    // Opens the db connection
    let db_connection = &mut DieselDB::database_connection();

    // The post ID to return as confirmation
    let post_id = post.id.clone();

    // Create the post on DB
    let result = diesel::insert_into(posts::table)
        .values(post)
        .execute(db_connection)
        .map(|_| post_id)
        .map_err(Error::DatabaseError);

    result
}
