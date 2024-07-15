mod create_user;
mod delete_user;
mod get_user_by_id;
mod get_users;
use actix_web::{web, Scope};

// This function returns all the routes
// related to the posts
pub fn users_routes() -> Scope {
    web::scope("users")
        .service(get_users::get_users)
        .service(create_user::create_user)
        .service(delete_user::delete_user)
        .service(get_user_by_id::get_user_by_id)
}
