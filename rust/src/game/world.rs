use crate::game::player::Player;

use super::collision::circles_collide;
use super::difficulty::Difficuly;
use super::enemy::Enemy;
use super::energy::Energy;
use super::light::{
    Light,
    ENERGY_CHARGE,
};
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
    light: Light,
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
            light: Light::new(),
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
        &self
    ) -> &Energy {
        &self.energy
    }
    // The lamp is the only light source in the
    // arena, so the browser asks the world how much
    // of it there is.
    pub fn light(
        &self
    ) -> &Light {
        &self.light
    }
    // Called when the player flips the lamp
    // switch. The server owns the lamp, the client
    // only reports the button press.
    pub fn set_light(
        &mut self,
        on: bool
    ) {
        self.light.set_on(on);
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
        self.light.reset();
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

        // Burn or recover lamp battery. This runs
        // before the enemies move so they react to
        // the lamp state of this tick.
        self.light.update(dt);

        // Update player position
        self.player.update(direction_x, direction_y, dt, self.width, self.height);

        // Update enemies
        //
        // An enemy caught in the beam moves faster
        // than one hiding in the dark, so the light
        // that lets you see them is also what
        // brings them down on you.
        let player_x = self.player.x();
        let player_y = self.player.y();

        for enemy in &mut self.enemies {
            let delta_x = enemy.x() - player_x;
            let delta_y = enemy.y() - player_y;

            let distance =
                (delta_x * delta_x + delta_y * delta_y).sqrt();

            let lure = self.light.enemy_lure_multiplier(distance);

            enemy.update(&self.player, dt, lure);
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
        //
        // Energy can only be picked up while the
        // lamp is burning. In the dark the orbs
        // are still there, you just cannot see them,
        // so they pass straight through the player.
        if self.light.is_lit() && circles_collide(
            self.player.x(), self.player.y(), self.player.size,
            self.energy.x, self.energy.y, self.energy.size,
        ) {
            let score_increment = if self.active_effects.has_double_score() { 20 } else { 10 };
            self.score += score_increment;

            // Every collected orb puts charge back
            // into the lamp.
            self.light.charge(ENERGY_CHARGE);

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
        self.powerups.check_collisions(
            &mut self.player,
            &mut self.active_effects,
            &mut self.light,
        );
    }

    fn respawn_energy(&mut self) {
        let mut rng = rand::thread_rng();
        self.energy.x = rng.gen_range(20.0..self.width - 20.0);
        self.energy.y = rng.gen_range(20.0..self.height - 20.0);
    }
}

#[cfg(test)]
mod tests {
    use super::super::light::{
        DRAIN_PER_SECOND,
        MAX_BATTERY,
    };
    use super::*;

    // The player and the first energy orb both
    // spawn in the middle of the arena, so a single
    // tick decides whether the orb can be picked up.
    const TICK: f32 = 1.0 / 60.0;

    // A test that runs several seconds of game time
    // needs an arena the enemies cannot cross in
    // that time, otherwise the player gets caught
    // and the run ends early.
    fn quiet_arena() -> Game {
        Game::with_dimensions(100_000.0, 100_000.0)
    }

    fn assert_close(
        actual: f32,
        expected: f32,
    ) {
        assert!(
            (actual - expected).abs() < 0.001,
            "expected {}, got {}",
            expected,
            actual
        );
    }

    #[test]
    fn energy_cannot_be_collected_in_the_dark() {
        let mut game = Game::new();

        game.set_light(false);
        game.start();
        game.update(0.0, 0.0, TICK);

        assert_eq!(game.score(), 0);
    }

    #[test]
    fn energy_can_be_collected_while_the_lamp_burns() {
        let mut game = Game::new();

        game.set_light(true);
        game.start();
        game.update(0.0, 0.0, TICK);

        assert_eq!(game.score(), 10);
    }

    #[test]
    fn collected_energy_puts_charge_back_in_the_lamp() {
        let mut game = quiet_arena();

        // Walk off the orb with the lamp off. This
        // is the blackout rule doing its job: no
        // light, no pickup.
        game.set_light(false);
        game.start();
        game.update(1.0, 0.0, 1.0);

        assert_eq!(game.score(), 0);

        // Light up and stand still. Ten seconds of
        // burning empties half the battery.
        game.set_light(true);
        game.update(0.0, 0.0, 10.0);

        assert_eq!(game.score(), 0);
        assert_close(
            game.light().battery(),
            MAX_BATTERY - DRAIN_PER_SECOND * 10.0
        );

        // Walk back over the orb, which is still
        // sitting on the spawn point.
        game.update(-1.0, 0.0, 1.0);

        assert_eq!(game.score(), 10);

        // Burning alone would have left the lamp
        // near 45, so anything above 60 can only be
        // the charge the orb gave back.
        assert!(
            game.light().battery() > 60.0,
            "lamp battery was {}",
            game.light().battery()
        );
    }

    #[test]
    fn the_lamp_dies_when_the_battery_runs_out() {
        let mut game = quiet_arena();

        game.start();

        // Twenty seconds of burning empties a full
        // battery.
        game.update(0.0, 0.0, 20.5);

        assert!(!game.light().is_on());
        assert!(!game.light().is_lit());
        assert_close(game.light().battery(), 0.0);
    }

    #[test]
    fn the_lamp_starts_every_run_charged_and_lit() {
        let mut game = quiet_arena();

        game.start();
        game.update(0.0, 0.0, 20.5);
        game.reset();

        assert!(game.light().is_lit());
        assert_close(game.light().battery(), MAX_BATTERY);
    }

    #[test]
    fn switching_the_lamp_off_is_a_server_side_decision() {
        let mut game = Game::new();

        game.set_light(false);

        assert!(!game.light().is_on());
    }
}
