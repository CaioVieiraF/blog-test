use actix_tests::DieselDB;
use actix_web::{get, web, HttpResponse};
use diesel::{prelude::*, SelectableHelper};
use uuid::Uuid;

use crate::models::{NewPost, Post};

#[get("/{id}")]
async fn get_post_by_id(path: web::Path<Uuid>) -> HttpResponse {
    use crate::schema::posts::dsl::posts;

    log::debug!("Querying post");

    // Open the DB connection
    let connection = &mut DieselDB::database_connection();
    // Get the post id from URL
    let post_id = path.into_inner();
    // Query the post
    let results = posts
        .find(String::from(post_id))
        .select(Post::as_select())
        .first(connection)
        .optional();

    match results {
        // If the post is found, then return
        // its id, title and body
        Ok(Some(result)) => HttpResponse::Ok().json(NewPost {
            id: result.id,
            title: result.title,
            body: result.body,
        }),
        // If it was not found, return 404 Not Found
        Ok(None) => HttpResponse::NotFound().finish(),
        // If the DB operation have some error,
        // return Internal Error
        Err(e) => {
            log::error!("Error querying post: {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
