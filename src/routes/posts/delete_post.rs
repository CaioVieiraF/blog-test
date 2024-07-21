use actix_web::{delete, web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use actix_tests::DieselDB;

use crate::prelude::*;

#[delete("{id}")]
async fn delete_post(path: web::Path<Uuid>) -> HttpResponse {
    // Delete the post
    let results = remove_post_from_db(path.into_inner());

    match results {
        // Return Ok if it was deleted successfully
        Ok(Some(result)) => {
            log::info!("Post deleted successfully.");
            log::debug!("Post {result}.");

            HttpResponse::Ok().finish()
        }

        // Return 404 Not Found if the post was not found
        Ok(None) => {
            log::info!("Post not found.");

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

fn remove_post_from_db(id: Uuid) -> Result<Option<usize>> {
    use crate::schema::posts::dsl::posts;

    // Open the DB connection
    let connection = &mut DieselDB::database_connection();

    // Delete the post from db
    diesel::delete(posts.find(String::from(id)))
        .execute(connection)
        .optional()
        .map_err(Error::DatabaseError)
}
