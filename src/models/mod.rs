use serde::Serialize;
use uuid::Uuid;

pub mod post;
pub mod user;

// Struct that matches the post creation response
#[derive(Serialize)]
pub struct CreationResponse {
    pub uuid: Uuid,
}
