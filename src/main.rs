mod topics_manager_client;
mod structs_definitions;

use axum::{response::IntoResponse, routing::get, Json, Router};
use structs_definitions::HealthStatus;

// Concurrent Processes


// Server setup

// Basic route to check if the server is online
async fn is_online() -> impl IntoResponse{
    let status: HealthStatus = HealthStatus { is_online: true };
    Json(status)
}

// Axum server router
fn init_router() -> Router{
    Router::new().route("/", get(is_online))
}

#[tokio::main]
async fn main(){
    // tokio::spawn(future());
    let app = init_router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("[ OK ]: MQTT Bridge is boomin... Listening on port 3000");
    axum::serve(listener, app).await.unwrap();
}   