use actix_web::{delete, web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use actix_tests::DieselDB;

use crate::prelude::*;

#[delete("{id}")]
async fn delete_user(path: web::Path<Uuid>) -> HttpResponse {
    let results = remove_user_from_db(path.into_inner());

    match results {
        // Return Ok if it was deleted successfully
        Ok(Some(result)) => {
            log::info!("User deleted successfully.");
            log::debug!("User {result}.");

            HttpResponse::Ok().finish()
        }

        // Return 404 Not Found if the post was not found
        Ok(None) => {
            log::info!("User not found.");

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

fn remove_user_from_db(user_id: Uuid) -> Result<Option<usize>> {
    use crate::schema::users::dsl::users;

    // Open the DB connection
    let connection = &mut DieselDB::database_connection();

    // Delete the user
    diesel::delete(users.find(String::from(user_id)))
        .execute(connection)
        .optional()
        .map_err(Error::DatabaseError)
}
