mod posts;
use actix_web::{web, Scope};

use self::posts::posts_routes;

// This function return the routes for all routes
// of this module
pub fn routes() -> Scope {
    web::scope("").service(posts_routes())
}
