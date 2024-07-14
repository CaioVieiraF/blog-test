use actix_web::{delete, web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use actix_tests::DieselDB;

#[delete("{id}")]
async fn delete_post(path: web::Path<Uuid>) -> HttpResponse {
    use crate::schema::posts::dsl::posts;

    // Get the post id from the URL
    let post_id = path.into_inner();
    // Open the DB connection
    let connection = &mut DieselDB::database_connection();

    // Delete the post
    let results = diesel::delete(posts.find(String::from(post_id)))
        .execute(connection)
        .optional();

    match results {
        // Return Ok if it was deleted successfully
        Ok(Some(result)) => {
            log::info!("Post with id {post_id} deleted successfully.");
            log::debug!("Post {result}.");

            HttpResponse::Ok().finish()
        }
        // Return 404 Not Found if the post was not found
        Ok(None) => {
            log::info!("Post with id {post_id} not found.");

            HttpResponse::NotFound().finish()
        }
        // Return Internal Server Error if the operation
        // did not work
        Err(e) => {
            log::error!("Error removing post: {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
