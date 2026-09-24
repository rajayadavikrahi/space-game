#[derive(Debug)]
pub struct Difficuly {
    enemy_spawn_interval: f32,
    enemy_speed_multiplier: f32
}

impl Difficuly {
    pub fn new() -> Self {
        Self::with_factor(1.0)
    }

    pub fn with_factor(factor: f32) -> Self {
        Self {
            enemy_spawn_interval: (3.0 / factor).clamp(1.5, 5.0),
            enemy_speed_multiplier: factor
        }
    }

    pub fn increase(&mut self) {
        self.enemy_spawn_interval *= 0.95;
        self.enemy_spawn_interval = self.enemy_spawn_interval.max(0.7);
        self.enemy_speed_multiplier *= 1.02;
        self.enemy_speed_multiplier = self.enemy_speed_multiplier.min(2.0);
    }

    pub fn spawn_interval(&self) -> f32 {
        self.enemy_spawn_interval
    }

    pub fn speed_multiplier(&self) -> f32 {
        self.enemy_speed_multiplier
    }

    pub fn enemy_speed(&self) -> f32 {
        100.0 * self.enemy_speed_multiplier
    }

    pub fn update(&mut self, elapsed_time: f32) {
        // Increase difficulty every 10 seconds
        if (elapsed_time % 10.0) < 0.1 {
            self.increase();
        }
    }

    pub fn reset(&mut self) {
        self.enemy_spawn_interval = 3.0;
        self.enemy_speed_multiplier = 1.0;
    }
}