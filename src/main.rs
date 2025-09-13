mod structs_definitions;
mod topics_manager_client;

use anyhow::{Error, Ok};
use axum::{Json, Router, response::IntoResponse, routing::get};
use dotenv::from_filename;
use std::env;
use structs_definitions::HealthStatus;
use topics_manager_client::TopicsManagerClient;

// Concurrent Processes

// Server setup
async fn topics_manager_workflow() -> Result<(), Error> {
    let username = env::var("USERNAME").unwrap();
    let password = env::var("PASSWORD").unwrap();
    let address = env::var("TM_ADDRESS").unwrap_or_else(|_| "http://localhost".into());
    let port = env::var("TM_PORT").unwrap_or_else(|_| "8080".into());

    let mut tmc = TopicsManagerClient::new(&username, &password, &address, &port);

    if let Err(e) = tmc.renew_auth_token().await {
        eprintln!("[ Error ]: Failed to renew auth token: {:?}", e);
        return Err(e);
    }

    let worked = tmc.has_auth_token().await;
    println!("Has token {:?}", worked);

    Ok(())
}

// Basic route to check if the server is online
async fn is_online() -> impl IntoResponse {
    let status: HealthStatus = HealthStatus { is_online: true };
    Json(status)
}

// Axum server router
fn init_router() -> Router {
    Router::new().route("/", get(is_online))
}

#[tokio::main]
async fn main() {
    let loaded = from_filename("src/.env").or_else(|_| from_filename(".env"));
    if loaded.is_ok() {
        println!("Loaded .env file");
    } else {
        println!("No .env file found in src/ or project root; reading environment directly");
    }

    tokio::spawn(topics_manager_workflow());
    let app = init_router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("[ OK ]: MQTT Bridge is boomin... Listening on port 3000");
    axum::serve(listener, app).await.unwrap();
}
