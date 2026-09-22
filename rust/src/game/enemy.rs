#[derive(Debug)]
pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub speed: f32
}

impl Enemy {
    pub fn new(
        x: f32,
        y: f32,
        speed: f32 
    ) -> Self {
        Self {
            x, y, size: 18.0, speed
        }
    }
    pub fn move_towards(
        &mut self,
        target_x: f32,
        target_y: f32,
        delta_time: f32
    ) {
        let dx = target_x - self.x;
        let dy = target_y - self.y;

        let distance = (dx * dx + dy * dy).sqrt();

        if distance == 0.0 {
            return;
        }
        let direction_x = dx/distance;
        let direction_y = dy/distance;
        self.x += direction_x * self.speed * delta_time;
        self.y += direction_y * self.speed * delta_time;
    }
}