use actix_tests::DieselDB;
use actix_web::{get, web, HttpResponse};
use diesel::{prelude::*, SelectableHelper};
use uuid::Uuid;

use crate::models::{
    post::Post,
    user::{UniqueUserInfo, User},
};

#[get("/{id}")]
async fn get_user_by_id(path: web::Path<Uuid>) -> HttpResponse {
    use crate::schema::users::dsl::users;

    log::debug!("Querying user");

    // Open the DB connection
    let connection = &mut DieselDB::database_connection();

    // Get the post id from URL
    let user_id = path.into_inner();

    // Query the post
    let results = users
        .find(String::from(user_id))
        .select(User::as_select())
        .first(connection)
        .optional();

    match results {
        // If the post is found, then return
        // its id, title and body
        Ok(Some(result)) => {
            let user_posts = Post::belonging_to(&result)
                .select(Post::as_select())
                .load(connection)
                .unwrap();
            HttpResponse::Ok().json(UniqueUserInfo {
                name: result.name,
                email: result.email,
                posts: user_posts,
            })
        }

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
