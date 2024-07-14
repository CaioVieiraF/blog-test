use actix_web::{put, web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use actix_tests::DieselDB;

#[put("{id}")]
async fn publish_post(path: web::Path<Uuid>) -> HttpResponse {
    use crate::schema::posts::dsl::{draft, posts};

    log::debug!("Publishing post");

    let id = path.into_inner();
    let connection = &mut DieselDB::database_connection();

    let update_operation = diesel::update(posts.find(String::from(id)))
        .set(draft.eq(false))
        .execute(connection);

    match update_operation {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}
