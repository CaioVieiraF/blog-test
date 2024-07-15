use actix_web::{put, web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use actix_tests::DieselDB;

#[put("{id}")]
async fn publish_post(path: web::Path<Uuid>) -> HttpResponse {
    use crate::schema::posts::dsl::{draft, posts};

    log::info!("Publishing post");

    // Get the id from the URL
    let id = path.into_inner();

    // Open the DB connection
    let connection = &mut DieselDB::database_connection();

    // Publish the post if it was not already
    let update_operation = diesel::update(posts.find(String::from(id)))
        .set(draft.eq(false))
        .execute(connection);

    match update_operation {
        // If there was no error, just return ok
        Ok(_) => HttpResponse::NoContent().finish(),

        // If there was an error, log it
        // and return an internal error
        Err(e) => {
            log::error!("Error updating the posts table: {e}");
            HttpResponse::BadRequest().finish()
        }
    }
}
