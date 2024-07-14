use crate::lib::DieselDB;
use crate::models::{self, *};
use actix_web::web::Json;
use actix_web::{get, web, Scope};
use diesel::{prelude::*, ExpressionMethods, SelectableHelper};

#[get("")]
async fn get_posts() -> Json<Vec<models::Post>> {
    use crate::schema::posts::dsl::*;

    log::debug!("Getting post");

    let connection = &mut DieselDB::database_connection();
    let results = posts
        .filter(public.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    Json(results)
}

pub fn posts_routes() -> Scope {
    web::scope("posts").service(get_posts)
}
