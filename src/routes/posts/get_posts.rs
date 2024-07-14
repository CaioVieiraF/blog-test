use actix_tests::DieselDB;
use actix_web::{get, HttpResponse};
use diesel::{prelude::*, SelectableHelper};

use crate::models::Post;

#[get("")]
async fn get_posts() -> HttpResponse {
    use crate::schema::posts::dsl::posts;

    log::debug!("Getting posts");

    // Open the DB connection
    let connection = &mut DieselDB::database_connection();
    // Get all the posts, even if there
    // is none.
    let results = posts.select(Post::as_select()).load(connection);

    match results {
        // Return all the posts
        Ok(result) => {
            log::info!("Getting all posts");

            HttpResponse::Ok().json(result)
        }
        // If there was an error accessing
        // the posts from the db, log the
        // error and return an internal error
        Err(e) => {
            log::error!("Error getting posts: {e}");

            HttpResponse::InternalServerError().finish()
        }
    }
}
