use actix_web::{put, web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use actix_tests::DieselDB;

use crate::prelude::*;

#[put("{id}")]
async fn publish_post(path: web::Path<Uuid>) -> HttpResponse {
    log::info!("Publishing post");

    // Publish the post if it was not already
    let update_operation = change_post_draft_status(path.into_inner());

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

fn change_post_draft_status(id: Uuid) -> Result<usize> {
    use crate::schema::posts::dsl::{draft, posts};

    // Open the DB connection
    let connection = &mut DieselDB::database_connection();

    // Change the post 'draft' field to false
    diesel::update(posts.find(String::from(id)))
        .set(draft.eq(false))
        .execute(connection)
        .map_err(Error::DatabaseError)
}
