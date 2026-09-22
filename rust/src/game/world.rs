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
    GameOver,
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

    elapsed_time: f32,
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
                8.0,
            ),

            score: 0,

            status: GameStatus::Waiting,

            width: 900.0,

            height: 600.0,

            enemy_spawn_timer: 0.0,

            enemy_spawn_interval: 3.0,

            elapsed_time: 0.0,
        }
    }

    // =========================
    // READ ONLY ACCESS
    // =========================

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

    pub fn enemy_spawn_interval(
        &self,
    ) -> f32 {
        self.enemy_spawn_interval
    }

    // =========================
    // GAME CONTROL
    // =========================

    pub fn start(&mut self) {
        if matches!(
            self.status,
            GameStatus::Waiting
        ) {
            self.status =
                GameStatus::Running;
        }
    }

    pub fn reset(&mut self) {
        self.player =
            Player::new(
                self.width / 2.0,
                self.height / 2.0,
            );

        self.enemies.clear();

        self.energy.move_to(
            self.width / 2.0,
            self.height / 2.0,
        );

        self.score = 0;

        self.enemy_spawn_timer = 0.0;

        self.enemy_spawn_interval = 3.0;

        self.elapsed_time = 0.0;

        self.status =
            GameStatus::Waiting;
    }

    // =========================
    // MAIN UPDATE
    // =========================

    pub fn update(
        &mut self,
        direction_x: f32,
        direction_y: f32,
        delta_time: f32,
    ) {
        if !matches!(
            self.status,
            GameStatus::Running
        ) {
            return;
        }

        self.elapsed_time +=
            delta_time;

        self.update_player(
            direction_x,
            direction_y,
            delta_time,
        );

        self.update_enemy_spawning(
            delta_time,
        );

        self.update_enemies(
            delta_time,
        );

        self.check_energy_collision();

        self.check_enemy_collision();
    }

    // =========================
    // PLAYER
    // =========================

    fn update_player(
        &mut self,
        direction_x: f32,
        direction_y: f32,
        delta_time: f32,
    ) {
        self.player.move_by(
            direction_x,
            direction_y,
            delta_time,
        );

        self.player.keep_inside(
            self.width,
            self.height,
        );
    }

    // =========================
    // ENEMY SPAWNING
    // =========================

    fn update_enemy_spawning(
        &mut self,
        delta_time: f32,
    ) {
        self.enemy_spawn_timer +=
            delta_time;

        if self.enemy_spawn_timer
            >= self.enemy_spawn_interval
        {
            self.spawn_enemy();

            self.enemy_spawn_timer =
                0.0;

            self.increase_difficulty();
        }
    }

    fn spawn_enemy(&mut self) {
        let mut rng =
            rand::thread_rng();

        let side =
            rng.gen_range(0..4);

        let (x, y) =
            match side {
                0 => {
                    let y =
                        rng.gen_range(
                            0.0..self.height,
                        );

                    (0.0, y)
                }

                1 => {
                    let y =
                        rng.gen_range(
                            0.0..self.height,
                        );

                    (self.width, y)
                }

                2 => {
                    let x =
                        rng.gen_range(
                            0.0..self.width,
                        );

                    (x, 0.0)
                }

                _ => {
                    let x =
                        rng.gen_range(
                            0.0..self.width,
                        );

                    (x, self.height)
                }
            };

        let speed =
            rng.gen_range(
                100.0..140.0,
            );

        self.enemies.push(
            Enemy::new(
                x,
                y,
                speed,
            ),
        );
    }

    fn increase_difficulty(
        &mut self,
    ) {
        self.enemy_spawn_interval =
            (
                self.enemy_spawn_interval
                    * 0.95
            )
            .max(0.7);
    }

    // =========================
    // ENEMIES
    // =========================

    fn update_enemies(
        &mut self,
        delta_time: f32,
    ) {
        let player_x =
            self.player.x;

        let player_y =
            self.player.y;

        for enemy
            in &mut self.enemies
        {
            enemy.move_towards(
                player_x,
                player_y,
                delta_time,
            );
        }
    }

    // =========================
    // ENERGY
    // =========================

    fn check_energy_collision(
        &mut self,
    ) {
        let collided =
            circles_collide(
                self.player.x,
                self.player.y,
                self.player.size,
                self.energy.x,
                self.energy.y,
                self.energy.size,
            );

        if collided {
            self.score += 10;

            self.spawn_energy();
        }
    }

    fn spawn_energy(
        &mut self,
    ) {
        let mut rng =
            rand::thread_rng();

        let x =
            rng.gen_range(
                self.energy.size
                    ..self.width
                        - self.energy.size,
            );

        let y =
            rng.gen_range(
                self.energy.size
                    ..self.height
                        - self.energy.size,
            );

        self.energy.move_to(
            x,
            y,
        );
    }

    // =========================
    // COLLISION
    // =========================

    fn check_enemy_collision(
        &mut self,
    ) {
        for enemy
            in &self.enemies
        {
            let collided =
                circles_collide(
                    self.player.x,
                    self.player.y,
                    self.player.size,
                    enemy.x,
                    enemy.y,
                    enemy.size,
                );

            if collided {
                self.status =
                    GameStatus::GameOver;

                return;
            }
        }
    }
}