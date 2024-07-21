use actix_tests::DieselDB;
use actix_web::web::Json;
use actix_web::{post, HttpResponse};
use bcrypt::{hash, DEFAULT_COST};
use diesel::RunQueryDsl;
use uuid::Uuid;

use crate::models::user::{NewUser, User};
use crate::models::CreationResponse;
use crate::prelude::*;

#[post("")]
async fn create_user(new_user_request: Json<NewUser>) -> HttpResponse {
    // Create the user object
    let new_user = new_user(new_user_request.into_inner());
    // Register the new user on the DB
    let results = register_user_on_db(new_user);

    match results {
        Ok(uuid) => {
            log::info!("Getting users.");
            HttpResponse::Ok().json(CreationResponse { uuid })
        }

        // If there was an error accessing
        // the users from the db, log the
        // error and return an internal error
        Err(e) => {
            log::error!("Error getting users: {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

fn new_user(info: NewUser) -> User {
    let password = hash(info.password.clone(), DEFAULT_COST).unwrap();

    User {
        id: Uuid::new_v4().to_string(),
        name: info.name.clone(),
        email: info.email.to_owned(),
        password,
    }
}

fn register_user_on_db(user: User) -> Result<String> {
    use crate::schema::users;

    // Open the DB connection
    let connection = &mut DieselDB::database_connection();

    // Get the user ID
    let id = user.id.clone();

    // Register the user on the DB
    diesel::insert_into(users::table)
        .values(user)
        .execute(connection)
        .map(|_| id)
        .map_err(Error::DatabaseError)
}
