use actix_web::{web, Scope};

mod create_post;
mod delete_post;
mod get_post_by_id;
mod get_posts;
mod publish_post;

pub fn posts_routes() -> Scope {
    web::scope("posts")
        .service(get_posts::get_posts)
        .service(create_post::create_post)
        .service(get_post_by_id::get_post_by_id)
        .service(publish_post::publish_post)
        .service(delete_post::delete_post)
}
