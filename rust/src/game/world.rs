use rand::Rng;

use super::{
    collision::circles_collide,
    energy::Energy,
    enemy::Enemy,
    player::Player,
};

#[derive(Debug, Clone, Copy)]
pub enum GameStatus {
    Waiting,
    Running, 
    GameOver
}
#[derive(Debug)]
pub struct Game {
    player: Player,
    enemies: Vec<Enemy>,
    energy: Energy,

    score: u32,
    status: GameStatus,
    width: f32,
    height: f32,
    enemy_spawn_timer: f32,
    enemy_spawn_interval: f32,
    elapsed_time: f32
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Player::new(
                450.0,
                300.0,
            ),
            enemies: Vec::new(),
            energy: Energy::new(
                200.0,
                200.0,
                8.0
            ),
            score: 0,
            status: GameStatus::Waiting,
            width: 900.0,
            height: 600.0,
            enemy_spawn_timer: 0.0,
            enemy_spawn_interval: 3.0,
            elapsed_time: 0.0
        }
    }

    pub fn player(&self) -> &Player {
        &self.player
    }
    pub fn enemies(&self) -> &[Enemy] {
        &self.enemies
    }
    pub fn energy(&self) -> &Energy {
        &self.energy
    }
    pub fn score(&self) -> u32 {
        self.score
    }
    pub fn status(&self) -> GameStatus {
        self.status
    }
    pub fn width(&self) -> f32 {
        self.width
    }
    pub fn height(&self) -> f32 {
        self.height
    }
    pub fn elapsed_time(&self) -> f32 {
        self.elapsed_time
    }
    pub fn enemy_spawn_interval(&self) -> f32 {
        self.enemy_spawn_interval
    }
    pub fn start(&mut self) {
        if matches!(
            self.status,
            GameStatus::Waiting
        ) {
            self.status = GameStatus::Running;
        }
    }
    pub fn reset(&mut self) {
        self.player = Player::new(
            self.width / 2.0,
            self.height / 2.0
        );
        self.enemies.clear();
        self.energy.move_to(
            self.width / 2.0,
            self.height / 2.0
        );
        self.score = 0;
        self.enemy_spawn_timer = 0;
        self.enemy_spawn_intercal = 3.0;
        self.elapsed_time = 0.0;
        self.status = GameStatus::Waiting;
    }
    pub fn update( &mut self, direction_x: f32, direction_y: f32, delta_time: f32) {
        if !matches!(
            self.status,
            GameStatus::Running
        ) {
            return;
        }
        self.elapsed_time += delta_time;

        self.update_player(
            direction_x,
            direction_y,
            delta_time
        );
        self.update_enemy_spawing(
            delta_time,
        );
        self.update_enemies(
            delta_time
        );
        self.check_energy_collision();
        self.check_enemy_collision();
    }
    fn update_player(
        &mut self,
        direction_x: f32,
        direction_y: f32,
        delta_time: f32
    ) {
        self.player.move_by(
            direction_x,
            direction_y,
            delta_time,
        );
        self.player.keep_inside(
            self.width,
            self.height
        );
    }
    fn update_enemy_spawing (
        &mut self,
        delta_time: f32,
    ) {
        self.enemy_spawn_timer += delta_time;
        if self.enemy_spawn_interval {
            self.spawn_enemy();
            self._enemy_spawn_timer = 0.0;
            self.increase_difficulty();
        }
    }
    
}

