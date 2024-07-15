use actix_tests::DieselDB;
use actix_web::{get, HttpResponse};
use diesel::query_dsl::methods::SelectDsl;
use diesel::{RunQueryDsl, SelectableHelper};

use crate::models::user::User;

#[get("")]
async fn get_users() -> HttpResponse {
    use crate::schema::users::dsl::users;
    // Open the DB connection
    let connection = &mut DieselDB::database_connection();

    // Getting all the users
    let results = users.select(User::as_select()).load(connection);

    match results {
        Ok(result) => {
            log::info!("Getting users.");
            HttpResponse::Ok().json(result)
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
