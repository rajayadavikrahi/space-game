mod api;
mod game;
mod server;

use std::sync::Arc;

use axum::{
    routing::get,
    Router,
};
use std::path::PathBuf;
use tower_http::services::ServeDir;

use tokio::sync::RwLock;

use api::handler::{
    websocket_handler,
    SharedGameServer,
};

use server::GameServer;

#[tokio::main]
async fn main() {

    // ======================================
    // CREATE GAME SERVER
    // ======================================

    let game_server =
        GameServer::new();

    // ======================================
    // SHARE SERVER STATE
    // ======================================

    let shared_server:
        SharedGameServer =
            Arc::new(
                RwLock::new(
                    game_server
                )
            );

    // ======================================
    // ROUTER
    // ======================================

    let client_path =
        PathBuf::from("../client");

    let app =
        Router::new()

            .route(
                "/ws",
                get(
                    websocket_handler
                ),
            )

            .fallback_service(
                ServeDir::new(client_path)
            )

            .with_state(
                shared_server
            );

    // ======================================
    // SERVER ADDRESS
    // ======================================

    let address =
        "127.0.0.1:3000";

    println!(
        "======================"
    );

    println!(
        "      VOID RUNNER"
    );

    println!(
        "======================"
    );

    println!();

    println!(
        "WebSocket: ws://{}/ws",
        address
    );

    println!();

    // ======================================
    // TCP LISTENER
    // ======================================

    let listener =
        tokio::net::TcpListener::bind(
            address
        )
        .await
        .unwrap();

    // ======================================
    // START SERVER
    // ======================================

    axum::serve(
        listener,
        app,
    )
    .await
    .unwrap();
}