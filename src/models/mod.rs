use serde::Serialize;

pub mod post;
pub mod user;

// Struct that matches the post creation response
#[derive(Serialize)]
pub struct CreationResponse {
    pub uuid: String,
}
