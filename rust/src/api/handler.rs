use axum::{
    extract::{
        State,
        WebSocketUpgrade,
        ws::{
            Message,
            WebSocket,
        },
    },
    response::IntoResponse,
};

use futures_util::{
    SinkExt,
    StreamExt,
};

use serde::{
    Deserialize,
    Serialize,
};

use std::sync::Arc;

use tokio::{
    sync::RwLock,
    time::{
        interval,
        Duration,
    },
};

use crate::{
    game::world::{
        Game,
        GameStatus,
    },
    server::{
        GameServer,
        PlayerId,
    },
};

pub type SharedGameServer =
    Arc<RwLock<GameServer>>;

// ==========================================
// CLIENT → SERVER
// ==========================================

#[derive(
    Debug,
    Deserialize,
)]
#[serde(
    tag = "type",
    rename_all = "snake_case"
)]
pub enum ClientMessage {
    Start,

    Reset,

    Input {
        direction_x: f32,
        direction_y: f32,
    },
}

// ==========================================
// SERVER → CLIENT
// ==========================================

#[derive(
    Debug,
    Serialize,
)]
#[serde(
    tag = "type",
    rename_all = "snake_case"
)]
pub enum ServerMessage {
    Connected {
        player_id: u64,
    },

    State {
        game: GameResponse,
    },
}

// ==========================================
// GAME RESPONSE
// ==========================================

#[derive(
    Debug,
    Serialize,
)]
pub struct GameResponse {
    pub score: u32,

    pub status: String,

    pub player: PositionResponse,

    pub energy: PositionResponse,

    pub enemies:
        Vec<PositionResponse>,
}

#[derive(
    Debug,
    Serialize,
)]
pub struct PositionResponse {
    pub x: f32,

    pub y: f32,

    pub size: f32,
}

// ==========================================
// WEBSOCKET ENTRY POINT
// ==========================================

pub async fn websocket_handler(
    ws: WebSocketUpgrade,

    State(server): State<
        SharedGameServer
    >,
) -> impl IntoResponse {
    ws.on_upgrade(
        move |socket| {
            handle_socket(
                socket,
                server,
            )
        },
    )
}

// ==========================================
// SOCKET
// ==========================================

async fn handle_socket(
    socket: WebSocket,

    server: SharedGameServer,
) {
    let (
        mut sender,
        mut receiver,
    ) = socket.split();

    // ======================================
    // CREATE PLAYER SESSION
    // ======================================

    let player_id = {
        let mut server =
            server.write().await;

        server.create_session()
    };

    // ======================================
    // SEND PLAYER ID
    // ======================================

    let connected_message =
        ServerMessage::Connected {
            player_id:
                player_id.value(),
        };

    let json =
        match serde_json::to_string(
            &connected_message,
        ) {
            Ok(json) => json,

            Err(error) => {
                eprintln!(
                    "Serialization error: {}",
                    error,
                );

                cleanup_session(
                    &server,
                    player_id,
                )
                .await;

                return;
            }
        };

    if sender
        .send(
            Message::Text(
                json.into()
            )
        )
        .await
        .is_err()
    {
        cleanup_session(
            &server,
            player_id,
        )
        .await;

        return;
    }

    // ======================================
    // CURRENT INPUT
    // ======================================

    let mut direction_x =
        0.0_f32;

    let mut direction_y =
        0.0_f32;

    // ======================================
    // GAME TICKER
    // ======================================

    let mut ticker =
        interval(
            Duration::from_millis(16)
        );

    // ======================================
    // MAIN SOCKET LOOP
    // ======================================

    loop {
        tokio::select! {

            // ==============================
            // CLIENT MESSAGE
            // ==============================

            message =
                receiver.next() => {

                match message {

                    Some(Ok(
                        Message::Text(text)
                    )) => {

                        handle_client_message(
                            &text,
                            &server,
                            player_id,
                            &mut direction_x,
                            &mut direction_y,
                        ).await;
                    }

                    Some(Ok(
                        Message::Close(_)
                    )) => {
                        break;
                    }

                    Some(Err(error)) => {

                        eprintln!(
                            "WebSocket error: {}",
                            error,
                        );

                        break;
                    }

                    None => {
                        break;
                    }

                    _ => {}
                }
            }

            // ==============================
            // GAME TICK
            // ==============================

            _ = ticker.tick() => {

                let response = {

                    let mut server =
                        server.write().await;

                    let game =
                        match server.game_mut(
                            player_id
                        ) {
                            Some(game) => game,

                            None => {
                                break;
                            }
                        };

                    game.update(
                        direction_x,
                        direction_y,
                        1.0 / 60.0,
                    );

                    game_response(game)
                };

                let message =
                    ServerMessage::State {
                        game: response,
                    };

                let json =
                    match serde_json::
                        to_string(&message)
                    {
                        Ok(json) => json,

                        Err(error) => {

                            eprintln!(
                                "Serialization error: {}",
                                error,
                            );

                            continue;
                        }
                    };

                if sender
                    .send(
                        Message::Text(
                            json.into()
                        )
                    )
                    .await
                    .is_err()
                {
                    break;
                }
            }
        }
    }

    // ======================================
    // DISCONNECT CLEANUP
    // ======================================

    cleanup_session(
        &server,
        player_id,
    )
    .await;
}

// ==========================================
// CLIENT MESSAGE HANDLER
// ==========================================

async fn handle_client_message(
    text: &str,

    server: &SharedGameServer,

    player_id: PlayerId,

    direction_x:
        &mut f32,

    direction_y:
        &mut f32,
) {
    let message =
        match serde_json::
            from_str::<ClientMessage>(
                text
            )
        {
            Ok(message) => message,

            Err(error) => {

                eprintln!(
                    "Invalid client message: {}",
                    error,
                );

                return;
            }
        };

    match message {

        // ==================================
        // START
        // ==================================

        ClientMessage::Start => {

            let mut server =
                server.write().await;

            if let Some(game) =
                server.game_mut(
                    player_id
                )
            {
                game.start();
            }
        }

        // ==================================
        // RESET
        // ==================================

        ClientMessage::Reset => {

            let mut server =
                server.write().await;

            if let Some(game) =
                server.game_mut(
                    player_id
                )
            {
                game.reset();

                *direction_x = 0.0;

                *direction_y = 0.0;
            }
        }

        // ==================================
        // INPUT
        // ==================================

        ClientMessage::Input {
            direction_x:
                input_x,

            direction_y:
                input_y,
        } => {

            *direction_x =
                input_x.clamp(
                    -1.0,
                    1.0,
                );

            *direction_y =
                input_y.clamp(
                    -1.0,
                    1.0,
                );
        }
    }
}

// ==========================================
// CLEANUP
// ==========================================

async fn cleanup_session(
    server: &SharedGameServer,

    player_id: PlayerId,
) {
    let mut server =
        server.write().await;

    server.remove_session(
        player_id
    );
}

// ==========================================
// DOMAIN → DTO
// ==========================================

fn game_response(
    game: &Game,
) -> GameResponse {

    let status =
        match game.status() {

            GameStatus::Waiting =>
                "waiting",

            GameStatus::Running =>
                "running",

            GameStatus::GameOver =>
                "game_over",
        };

    GameResponse {
        score:
            game.score(),

        status:
            status.to_string(),

        player:
            PositionResponse {
                x:
                    game.player().x,

                y:
                    game.player().y,

                size:
                    game.player().size,
            },

        energy:
            PositionResponse {
                x:
                    game.energy().x,

                y:
                    game.energy().y,

                size:
                    game.energy().size,
            },

        enemies:
            game.enemies()
                .iter()
                .map(
                    |enemy| {
                        PositionResponse {
                            x:
                                enemy.x,

                            y:
                                enemy.y,

                            size:
                                enemy.size,
                        }
                    }
                )
                .collect(),
    }
}