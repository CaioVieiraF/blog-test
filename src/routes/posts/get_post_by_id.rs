use actix_tests::DieselDB;
use actix_web::{get, web, HttpResponse};
use diesel::{prelude::*, SelectableHelper};
use uuid::Uuid;

use crate::{
    models::post::{NewPost, Post},
    prelude::*,
};

#[get("{id}")]
async fn get_post_by_id(path: web::Path<Uuid>) -> HttpResponse {
    log::debug!("Querying post");

    // Query the post
    let results = get_post_from_db(path.into_inner());

    match results {
        // If the post is found, then return
        // its id, title and body
        Ok(Some(result)) => HttpResponse::Ok().json(NewPost {
            id: result.id,
            title: result.title,
            body: result.body,
            user_id: result.user_id,
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

fn get_post_from_db(id: Uuid) -> Result<Option<Post>> {
    use crate::schema::posts::dsl::posts;

    // Open the DB connection
    let connection = &mut DieselDB::database_connection();

    posts
        .find(String::from(id))
        .select(Post::as_select())
        .first(connection)
        .optional()
        .map_err(Error::DatabaseError)
}
