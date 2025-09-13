use serde::{Serialize};

// main

#[derive(Serialize)]
pub struct HealthStatus{
    pub is_online: bool,
}