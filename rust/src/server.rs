use std::collections::HashMap;

use crate::game::world::Game;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct PlayerId(u64);

impl PlayerId {
    pub fn value(&self) -> u64 {
        self.0
    }
}

#[derive(Debug)]
pub struct Session {
    pub game: Game,
}

impl Session {
    pub fn new() -> Self {
        Self {
            game: Game::new(),
        }
    }
}

#[derive(Debug)]
pub struct GameServer {
    next_player_id: u64,

    sessions:
        HashMap<PlayerId, Session>,
}

impl GameServer {
    pub fn new() -> Self {
        Self {
            next_player_id: 1,

            sessions:
                HashMap::new(),
        }
    }

    pub fn create_session(
        &mut self,
    ) -> PlayerId {
        let player_id =
            PlayerId(
                self.next_player_id
            );

        self.next_player_id += 1;

        self.sessions.insert(
            player_id,
            Session::new(),
        );

        println!(
            "Player {} connected",
            player_id.value()
        );

        player_id
    }

    pub fn remove_session(
        &mut self,
        player_id: PlayerId,
    ) {
        self.sessions
            .remove(&player_id);

        println!(
            "Player {} disconnected",
            player_id.value()
        );
    }

    pub fn game_mut(
        &mut self,
        player_id: PlayerId,
    ) -> Option<&mut Game> {
        self.sessions
            .get_mut(&player_id)
            .map(|session| {
                &mut session.game
            })
    }

    pub fn game(
        &self,
        player_id: PlayerId,
    ) -> Option<&Game> {
        self.sessions
            .get(&player_id)
            .map(|session| {
                &session.game
            })
    }

    pub fn session_count(
        &self,
    ) -> usize {
        self.sessions.len()
    }
}