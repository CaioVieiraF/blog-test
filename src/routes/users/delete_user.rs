use actix_web::{delete, web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use actix_tests::DieselDB;

#[delete("{id}")]
async fn delete_user(path: web::Path<Uuid>) -> HttpResponse {
    use crate::schema::users::dsl::users;

    // Get the post id from the URL
    let user_id = path.into_inner();

    // Open the DB connection
    let connection = &mut DieselDB::database_connection();

    // Delete the post
    let results = diesel::delete(users.find(String::from(user_id)))
        .execute(connection)
        .optional();

    match results {
        // Return Ok if it was deleted successfully
        Ok(Some(result)) => {
            log::info!("User with id {user_id} deleted successfully.");
            log::debug!("User {result}.");

            HttpResponse::Ok().finish()
        }

        // Return 404 Not Found if the post was not found
        Ok(None) => {
            log::info!("User with id {user_id} not found.");

            HttpResponse::NotFound().finish()
        }

        // Return Internal Server Error if the operation
        // did not work
        Err(e) => {
            log::error!("Error removing user: {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
