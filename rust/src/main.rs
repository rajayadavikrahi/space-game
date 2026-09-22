mod api;
mod game;

use std::sync::{
    Arc, 
    Mutex
};

use axum::{
    http::Method,
    routing::{get, post},
    Router
};

use tower_http::cors::{
    Any,
    CorsLayer
};

use api::handlers::{
    get_game,
    move_player,
    reset_game,
    start_game,
    websocket_handler,
    SharedGame
};
use game::world::Game;

#[tokio::main]
async fn main() {
    let game = Game::new();

    let shared_game: SharedGame = Arc::new(Mutex::new(game));
    let cors = CorsLayer::new().allow_origin(Any).allow_methods([
        Method::GET,
        Method::POST
    ])
    .allow_headers(Any);

    let app = Router::new()
        .route(
            "/api/game",
            get(get_game),
        )
        .route(
            "/api/game/start",
            post(start_game),
        )
        .route(
            "/api/game/reset",
            post(reset_game),
        )
        .route(
            "/api/game/move",
            post(move_player),
        )
        .route(
            "/ws",
            get(websocket_handler),
        )
        .layer(cors)
        .with_state(shared_game);

    let address = "127.0.0.1:3000";

    println!("======================");
    println!("     VOID RUNNER");
    println!("======================");
    println!();
    println!(
        "HTTP:      http://{}",
        address
    );
    println!(
        "WebSocket: ws://{}/ws",
        address
    );

    let listener =
        tokio::net::TcpListener::bind(address)
            .await
            .unwrap();

    axum::serve(
        listener,
        app,
    )
    .await
    .unwrap();
}
}