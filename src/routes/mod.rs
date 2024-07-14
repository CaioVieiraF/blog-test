mod posts;
use actix_web::{web, Scope};

use self::posts::posts_routes;

pub fn routes() -> Scope {
    web::scope("").service(posts_routes())
}
