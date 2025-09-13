use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct AuthPayload{
    pub username: String,
    pub password: String
}