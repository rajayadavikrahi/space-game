use rand::Rng;
use super::{
    kind::PowerUpKind,
    powerup::PowerUp
};

#[derive(Debug)]
pub struct PowerUpSpawner {
    timer: f32,
    interval: f32,
}

impl PowerUpSpawner {
    pub fn new() -> Self {
        Self {
            timer: 0.0,
            interval: 10.0
        }
    }

    pub fn update(
        &mut self,
        delta_time: f32,
        width: f32,
        height: f32
    ) -> Option<PowerUp> {
        self.timer += delta_time;
        if self.timer < self.interval {
            return None;
        }
        self.timer = 0.0;
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(20.0..width - 20.0);
        let y = rng.gen_range(20.0..height - 20.0);
        let kind_number = rng.gen_range(0..3);
        let kind = match kind_number {
            0 => PowerUpKind::Shield,

            // If the number is 1:
            // create a Speed power-up.
            1 => PowerUpKind::Speed,

            // `_` means:
            // "anything else."
            //
            // Since the only remaining possibility
            // is 2, this becomes DoubleScore.
            _ => PowerUpKind::DoubleScore,
        };
        let duration = 5.0;
        let power_up = PowerUp::new(x, y, kind, duration);
        Some(power_up)
    }

    pub fn interval(&self) -> f32 {
        self.interval
    }

    pub fn reset(&mut self) {
        self.timer = 0.0;
    }
}