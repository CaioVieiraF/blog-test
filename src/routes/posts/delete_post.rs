use actix_web::{delete, web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use actix_tests::DieselDB;

#[delete("{id}")]
async fn delete_post(path: web::Path<Uuid>) -> HttpResponse {
    use crate::schema::posts::dsl::posts;

    let post_id = path.into_inner();
    let connection = &mut DieselDB::database_connection();

    let results = diesel::delete(posts.find(String::from(post_id)))
        .execute(connection)
        .optional();

    match results {
        Ok(Some(result)) => {
            log::info!("Post with id {post_id} deleted successfully.");
            log::debug!("Post {result}.");

            HttpResponse::Ok().finish()
        }
        Ok(None) => {
            log::debug!("Post with id {post_id} not found.");

            HttpResponse::NotFound().finish()
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
