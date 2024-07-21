use actix_tests::DieselDB;
use actix_web::{get, web, HttpResponse};
use diesel::{prelude::*, SelectableHelper};
use uuid::Uuid;

use crate::{
    models::{
        post::Post,
        user::{UniqueUserInfo, User},
    },
    prelude::*,
};

#[get("/{id}")]
async fn get_user_by_id(path: web::Path<Uuid>) -> HttpResponse {
    log::debug!("Querying user");

    // Open the DB connection
    let connection = &mut DieselDB::database_connection();
    // Query the post
    let results = get_user_from_db(path.into_inner(), connection);

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

fn get_user_from_db(user_id: Uuid, connection: &mut SqliteConnection) -> Result<Option<User>> {
    use crate::schema::users::dsl::users;

    users
        .find(String::from(user_id))
        .select(User::as_select())
        .first(connection)
        .optional()
        .map_err(Error::DatabaseError)
}
