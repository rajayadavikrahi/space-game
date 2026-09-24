use crate::game::player::Player;

use super::collision::circles_collide;
use super::difficulty::Difficuly;
use super::enemy::Enemy;
use super::energy::Energy;
use super::powerup::manager::PowerUpManager;
use super::powerup::state::ActiveEffects;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
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
    powerups: PowerUpManager,
    active_effects: ActiveEffects,
    difficulty: Difficuly,
    difficulty_factor: f32,
    score: u32,
    status: GameStatus,
    width: f32,
    height: f32,
    enemy_spawn_timer: f32,
    elapsed_time: f32,

}

impl Game {
    pub fn new() -> Self {
        Self::with_dimensions(900.0, 600.0)
    }

    pub fn with_dimensions(
        width: f32,
        height: f32,
    ) -> Self {
        Self {
            player: Player::new(
                width / 2.0,
                height / 2.0
            ),
            enemies: Vec::new(),
            energy: Energy::new(
                width / 2.0,
                height / 2.0,
                10.0
            ),
            powerups: PowerUpManager::new(),
            active_effects: ActiveEffects::new(),
            difficulty:Difficuly::new(),
            difficulty_factor: 1.0,
            score: 0,
            status: GameStatus::Waiting,
            width,
            height,
            enemy_spawn_timer: 0.0,
            elapsed_time: 0.0
        }
    }
    pub fn player(
        &self,
    ) -> &Player {
        &self.player
    } 
    pub fn energy(
        &self,
    ) -> &Energy {
        &self.energy
    }
    pub fn powerups(
        &self,
    ) -> &PowerUpManager {
        &self.powerups
    }
    pub fn enemies(
        &self,
    ) -> &[Enemy] {
        &self.enemies
    }
    pub fn active_effects(
        &self
    ) -> &ActiveEffects {
        &self.active_effects
    }
    pub fn score(
        &self
    ) -> u32 {
        self.score
    }
    pub fn status(
        &self,
    ) -> GameStatus {
        self.status
    }
    pub fn set_difficulty(
        &mut self,
        difficulty: &str,
    ) {
        let factor =
            match difficulty {
                "easy" => 0.8,
                "hard" => 1.4,
                _ => 1.0,
            };

        self.difficulty_factor = factor;
        self.difficulty = Difficuly::with_factor(factor);
        self.enemy_spawn_timer = 0.0;
    }
    pub fn active_powerup(
        &self,
    ) -> Option<String> {
        let effects = &self.active_effects;
        if effects.has_shield() {
            Some(
                "shield"
                    .to_string(),
            )
        } else if effects
            .has_speed()
        {
            Some(
                "speed"
                    .to_string(),
            )
        } else if effects
            .has_double_score()
        {
            Some(
                "double_score"
                    .to_string(),
            )
        } else {
            None
        }
    } 
    pub fn width(
        &self,
    ) -> f32 {
        self.width
    }
    pub fn height(&self) -> f32 {
        self.height
    }
    pub fn elapsed_time(
        &self
    ) -> f32 {
        self.elapsed_time
    } 
    pub fn start(
        &mut self
    ) {
        if self.status == GameStatus::Waiting {
            self.status = GameStatus::Running;
        }
    }
    pub fn reset(
        &mut self
    ) {
        self.player = Player::new(
            self.width / 2.0,
            self.height / 2.0
        );
        self.enemies.clear();
        self.energy = Energy::new(
            self.width / 2.0,
            self.height / 2.0,
            10.0
        );
        self.powerups = PowerUpManager::new();
        self.active_effects = ActiveEffects::new();
        self.difficulty = Difficuly::with_factor(self.difficulty_factor);
        self.score = 0;
        self.status = GameStatus::Waiting;
        self.enemy_spawn_timer = 0.0;
        self.elapsed_time = 0.0;

    }

    pub fn update(
        &mut self,
        direction_x: f32,
        direction_y: f32,
        dt: f32,
    ) {
        if self.status != GameStatus::Running {
            return;
        }

        self.elapsed_time += dt;

        // Update player position
        self.player.update(direction_x, direction_y, dt, self.width, self.height);

        // Update enemies
        for enemy in &mut self.enemies {
            enemy.update(&self.player, dt);
        }

        // Spawn enemies
        self.enemy_spawn_timer += dt;
        if self.enemy_spawn_timer > self.difficulty.spawn_interval() {
            self.spawn_enemy();
            self.enemy_spawn_timer = 0.0;
        }

        // Check collisions
        self.check_collisions();

        // Update difficulty
        self.difficulty.update(self.elapsed_time);

        // Update powerups
        self.powerups.update(dt, self.width, self.height);

        // Update active effects
        self.active_effects.update(dt);
    }

    fn spawn_enemy(&mut self) {
        let mut rng = rand::thread_rng();
        let side = rng.gen_range(0..4);
        let (x, y) = match side {
            0 => (rng.gen_range(0.0..self.width), -20.0), // top
            1 => (rng.gen_range(0.0..self.width), self.height + 20.0), // bottom
            2 => (-20.0, rng.gen_range(0.0..self.height)), // left
            3 => (self.width + 20.0, rng.gen_range(0.0..self.height)), // right
            _ => (0.0, 0.0),
        };

        let enemy = Enemy::new(x, y, self.difficulty.enemy_speed());
        self.enemies.push(enemy);
    }

    fn check_collisions(&mut self) {
        // Player-energy collision
        if circles_collide(
            self.player.x(), self.player.y(), self.player.size,
            self.energy.x, self.energy.y, self.energy.size,
        ) {
            let score_increment = if self.active_effects.has_double_score() { 20 } else { 10 };
            self.score += score_increment;
            self.respawn_energy();
        }

        // Player-enemy collisions
        for enemy in &self.enemies {
            if circles_collide(
                self.player.x(), self.player.y(), self.player.size,
                enemy.x(), enemy.y(), enemy.size,
            ) {
                if !self.active_effects.has_shield() {
                    self.status = GameStatus::GameOver;
                }
            }
        }

        // Player-powerup collisions
        self.powerups.check_collisions(&mut self.player, &mut self.active_effects, &mut self.score);
    }

    fn respawn_energy(&mut self) {
        let mut rng = rand::thread_rng();
        self.energy.x = rng.gen_range(20.0..self.width - 20.0);
        self.energy.y = rng.gen_range(20.0..self.height - 20.0);
    }
}