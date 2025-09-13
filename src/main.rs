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
    let username: String = env::var("USERNAME").unwrap();
    let password: String = env::var("PASSWORD").unwrap();
    let address: String = env::var("TM_ADDRESS").unwrap_or_else(|_| "http://localhost".into());
    let port: i32 = env::var("TM_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);

    let mut tmc = TopicsManagerClient::new(&username, &password, &address, port);

    if let Err(e) = tmc.renew_auth_token().await {
        eprintln!("[ Error ]: Failed to renew auth token: {:?}", e);
        return Err(e);
    }

    if let Err(e) = tmc.register().await {
        eprintln!("[ Error ]: Failed to register: {:?}", e);
        return Err(e);
    }
    

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
        println!("[ LOG ]: Loaded .env file");
    } else {
        println!("[ ERROR ]: No .env file found in src/ or project root; reading environment directly");
    }

    tokio::spawn(topics_manager_workflow());
    let app = init_router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("[ LOG ]: MQTT Bridge is boomin... Listening on port 3000");
    axum::serve(listener, app).await.unwrap();
}
