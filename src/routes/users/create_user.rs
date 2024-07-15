use actix_tests::DieselDB;
use actix_web::web::Json;
use actix_web::{post, HttpResponse};
use diesel::RunQueryDsl;
use uuid::Uuid;

use crate::models::user::{NewUser, User};
use crate::models::CreationResponse;

#[post("")]
async fn create_user(new_user: Json<NewUser>) -> HttpResponse {
    use crate::schema::users;
    // Open the DB connection
    let connection = &mut DieselDB::database_connection();

    let user_id = Uuid::new_v4();

    let new_user = User {
        id: user_id.to_string(),
        name: new_user.name.clone(),
        email: new_user.email.clone(),
        password: new_user.password.to_owned(),
    };

    // Getting all the users
    // let results = users.select(User::as_select()).load(connection);
    let results = diesel::insert_into(users::table)
        .values(new_user)
        .execute(connection);

    match results {
        Ok(_) => {
            log::info!("Getting users.");
            HttpResponse::Ok().json(CreationResponse { uuid: user_id })
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
