use axum::{
    extract::{
        State,
        WebSocketUpgrade,
        ws::{
            Message,
            WebSocket,
        },
    },
    http::StatusCode,
    response::Json,
};

use futures_util::{
    SinkExt,
    StreamExt,
};

use serde::{
    Deserialize,
    Serialize,
};

use std::sync::{
    Arc,
    Mutex,
};

use tokio::time::{
    interval,
    Duration,
};

use crate::game::world::{
    Game,
    GameStatus,
};

pub type SharedGame = Arc<Mutex<Game>>;

// --------------------------------------------------
// RESPONSE TYPES
// --------------------------------------------------

#[derive(Debug, Serialize)]
pub struct GameResponse {
    pub score: u32,
    pub status: String,
    pub player: PositionResponse,
    pub energy: PositionResponse,
    pub enemies: Vec<PositionResponse>,
}

#[derive(Debug, Serialize)]
pub struct PositionResponse {
    pub x: f32,
    pub y: f32,
    pub size: f32,
}

// --------------------------------------------------
// REQUEST TYPES
// --------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct MoveRequest {
    pub direction_x: f32,
    pub direction_y: f32,
}

// --------------------------------------------------
// REST API
// --------------------------------------------------

pub async fn get_game(
    State(game): State<SharedGame>,
) -> Json<GameResponse> {
    let game = game.lock().unwrap();

    Json(game_response(&game))
}

pub async fn start_game(
    State(game): State<SharedGame>,
) -> Json<GameResponse> {
    let mut game = game.lock().unwrap();

    game.start();

    Json(game_response(&game))
}

pub async fn reset_game(
    State(game): State<SharedGame>,
) -> Json<GameResponse> {
    let mut game = game.lock().unwrap();

    game.reset();

    Json(game_response(&game))
}

pub async fn move_player(
    State(game): State<SharedGame>,
    Json(request): Json<MoveRequest>,
) -> Result<Json<GameResponse>, StatusCode> {
    let mut game = game.lock().unwrap();

    if request.direction_x.abs() > 1.0
        || request.direction_y.abs() > 1.0
    {
        return Err(StatusCode::BAD_REQUEST);
    }

    let delta_time = 1.0 / 60.0;

    game.update(
        request.direction_x,
        request.direction_y,
        delta_time,
    );

    Ok(Json(game_response(&game)))
}

// --------------------------------------------------
// WEBSOCKET
// --------------------------------------------------

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(game): State<SharedGame>,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |socket| {
        handle_socket(socket, game)
    })
}

async fn handle_socket(
    socket: WebSocket,
    game: SharedGame,
) {
    let (mut sender, mut receiver) =
        socket.split();

    let mut direction_x = 0.0_f32;
    let mut direction_y = 0.0_f32;

    let mut ticker =
        interval(Duration::from_millis(16));

    loop {
        tokio::select! {

            message = receiver.next() => {
                match message {
                    Some(Ok(Message::Text(text))) => {

                        match serde_json::from_str::<MoveRequest>(&text) {
                            Ok(input) => {
                                direction_x =
                                    input.direction_x.clamp(-1.0, 1.0);

                                direction_y =
                                    input.direction_y.clamp(-1.0, 1.0);
                            }

                            Err(error) => {
                                eprintln!(
                                    "Invalid WebSocket message: {}",
                                    error
                                );
                            }
                        }
                    }

                    Some(Ok(Message::Close(_))) => {
                        break;
                    }

                    Some(Err(error)) => {
                        eprintln!(
                            "WebSocket error: {}",
                            error
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
                    let mut game =
                        game.lock().unwrap();

                    game.update(
                        direction_x,
                        direction_y,
                        1.0 / 60.0,
                    );

                    game_response(&game)
                };

                let json =
                    match serde_json::to_string(&response) {
                        Ok(json) => json,

                        Err(error) => {
                            eprintln!(
                                "Serialization error: {}",
                                error
                            );

                            continue;
                        }
                    };

                if sender
                    .send(Message::Text(json.into()))
                    .await
                    .is_err()
                {
                    break;
                }
            }
        }
    }
}

// --------------------------------------------------
// GAME → API RESPONSE
// --------------------------------------------------

fn game_response(game: &Game) -> GameResponse {
    let status = match game.status {
        GameStatus::Waiting => "waiting",
        GameStatus::Running => "running",
        GameStatus::GameOver => "game_over",
    };

    GameResponse {
        score: game.score,

        status: status.to_string(),

        player: PositionResponse {
            x: game.player.x,
            y: game.player.y,
            size: game.player.size,
        },

        energy: PositionResponse {
            x: game.energy.x,
            y: game.energy.y,
            size: game.energy.size,
        },

        enemies: game
            .enemies
            .iter()
            .map(|enemy| PositionResponse {
                x: enemy.x,
                y: enemy.y,
                size: enemy.size,
            })
            .collect(),
    }
}