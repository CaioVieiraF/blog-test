use actix_tests::DieselDB;
use actix_web::{get, web, HttpResponse};
use diesel::{prelude::*, SelectableHelper};
use uuid::Uuid;

use crate::models::{NewPost, Post};

#[get("/{id}")]
async fn get_post_by_id(path: web::Path<Uuid>) -> HttpResponse {
    use crate::schema::posts::dsl::posts;

    log::debug!("Querying post");

    let connection = &mut DieselDB::database_connection();
    let post_id = path.into_inner();
    let results = posts
        .find(String::from(post_id))
        .select(Post::as_select())
        .first(connection)
        .optional();

    match results {
        Ok(Some(result)) => HttpResponse::Ok().json(NewPost {
            id: result.id,
            title: result.title,
            body: result.body,
        }),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
