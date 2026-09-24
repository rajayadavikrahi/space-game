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

    Difficulty {
        difficulty: String,
    },
}

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

    pub powerups:
        Vec<PowerUpResponse>,

    pub active_powerup:
        Option<String>,
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

#[derive(
    Debug,
    Serialize,
)]
pub struct PowerUpResponse {
    pub x: f32,

    pub y: f32,

    pub size: f32,

    pub kind: String,

    pub duration: f32,
}

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

async fn handle_socket(
    socket: WebSocket,
    server: SharedGameServer,
) {
    let (
        mut sender,
        mut receiver,
    ) = socket.split();

    let player_id = {
        let mut server =
            server.write().await;

        server.create_session()
    };

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

    let mut direction_x =
        0.0_f32;

    let mut direction_y =
        0.0_f32;

    let mut ticker =
        interval(
            Duration::from_millis(16)
        );

    loop {
        tokio::select! {

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

    cleanup_session(
        &server,
        player_id,
    )
    .await;
}

async fn handle_client_message(
    text: &str,
    server: &SharedGameServer,
    player_id: PlayerId,
    direction_x: &mut f32,
    direction_y: &mut f32,
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

        ClientMessage::Difficulty {
            difficulty,
        } => {
            let mut server =
                server.write().await;

            if let Some(game) =
                server.game_mut(
                    player_id
                )
            {
                game.set_difficulty(
                    &difficulty,
                );
            }
        }
    }
}

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

    let powerups =
        game.powerups()
            .iter()
            .map(
                |powerup| {
                    let kind =
                        match powerup.kind {
                            crate::game::powerup::kind::PowerUpKind::Shield =>
                                "shield",

                            crate::game::powerup::kind::PowerUpKind::Speed =>
                                "speed",

                            crate::game::powerup::kind::PowerUpKind::DoubleScore =>
                                "double_score",
                        };

                    PowerUpResponse {
                        x:
                            powerup.x,

                        y:
                            powerup.y,

                        size:
                            powerup.size,

                        kind:
                            kind.to_string(),

                        duration:
                            powerup.duration,
                    }
                }
            )
            .collect();

    GameResponse {
        score:
            game.score(),

        status:
            status.to_string(),

        player:
            PositionResponse {
                x:
                    game.player().x(),

                y:
                    game.player().y(),

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
                                enemy.x(),

                            y:
                                enemy.y(),

                            size:
                                enemy.size,
                        }
                    }
                )
                .collect(),

        powerups,

        active_powerup:
            game.active_powerup(),
    }
}