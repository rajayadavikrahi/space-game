use super::kind::PowerUpKind;

#[derive(Debug)]
pub struct ActiveEffects {
    shield_remaining: Option<f32>,
    speed_remaining: Option<f32>,
    double_score_remaining: Option<f32>
}

impl ActiveEffects {
    pub fn new() -> Self {
        Self {
            shield_remaining: None,
            speed_remaining: None,
            double_score_remaining: None
        }
    }
    pub fn activate(
        &mut self,
        kind: PowerUpKind,
        duration: f32
    ) {
        match kind {
            PowerUpKind::Shield => {
                self.shield_remaining = Some(duration);
            }
            PowerUpKind::Speed => {
                self.speed_remaining = Some(duration);
            }
            PowerUpKind::DoubleScore => {
                self.double_score_remaining = Some(duration);
            }
        }
    }
    pub fn update(
        &mut self,
        delta_time: f32,
    ) {
        if let Some(remaining) = self.shield_remaining.as_mut() {
            *remaining -= delta_time;
            if *remaining <= 0.0 {
                self.shield_remaining = None;
            }
        }
        if let Some(
            remaining
        ) = self.speed_remaining.as_mut() {
            *remaining -= delta_time;
            if *remaining <= 0.0 {
                self.speed_remaining = None;
            }
        }
        if let Some(
            remaining
        ) = self.double_score_remaining.as_mut() {
            *remaining -= delta_time;
            if *remaining <= 0.0 {
                self.double_score_remaining = None;
            }
        }
    }
    pub fn has_shield(
        &self,
    ) -> bool{
        self.shield_remaining.is_some()
    }
    pub fn has_speed(
        &self
    ) -> bool {
        self.speed_remaining.is_some()
    }
    pub fn has_double_score(
        &self
    ) -> bool {
        self.double_score_remaining.is_some()
    }
    pub fn consume_shield(&mut self) -> bool {
        if self.shield_remaining.is_some() {
            self.shield_remaining = None;
            true
        } else{
            false
        }
    }
    pub fn shield_remaining(
        &self
    ) -> Option<f32> {
        self.shield_remaining
    }
    pub fn speed_remaining(
        &self
    ) -> Option<f32> {
        self.speed_remaining
    }
    pub fn double_score_remaining(
        &self
    ) -> Option<f32> {
        self.double_score_remaining
    }

    pub fn clear(
        &mut self
    ) {
        self.shield_remaining = None;
        self.speed_remaining = None;
        self.double_score_remaining = None;
    }
 } 