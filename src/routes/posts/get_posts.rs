use actix_tests::DieselDB;
use actix_web::{get, HttpResponse};
use diesel::{prelude::*, SelectableHelper};

use crate::models::Post;

#[get("")]
async fn get_posts() -> HttpResponse {
    use crate::schema::posts::dsl::posts;

    log::debug!("Getting posts");

    let connection = &mut DieselDB::database_connection();
    let results = posts
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    HttpResponse::Ok().json(results)
}
