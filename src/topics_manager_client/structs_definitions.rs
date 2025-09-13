use serde::{Serialize};

#[derive(Serialize)]
pub struct AuthPayload{
    pub username: String,
    pub password: String
}

#[derive(Serialize)]
pub struct RegisterPayload{
    pub host: String,
    pub port: i32
}