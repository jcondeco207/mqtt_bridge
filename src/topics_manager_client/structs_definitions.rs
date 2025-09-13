use serde::{Serialize};

#[derive(Serialize)]
pub struct AuthPayload{
    pub username: String,
    pub password: String
}