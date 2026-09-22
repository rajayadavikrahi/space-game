#[derive(Debug)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub speed: f32
}
impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x, y, size: 20.0, speed: 300.0
        }
    }

    pub fn move_by(
        &mut self,
        direction_x: f32,
        direction_y: f32,
        delta_time: f32
    ) {
        self.x +=
            direction_x * self.speed * delta_time;
        self.y +=
            direction_y * self.speed * delta_time;
    }

    pub fn keep_inside(
        &mut self,
        width: f32,
        height: f32
    ) {
        self.x = self
            .x
            .clamp(
                self.size,
                width - self.size,
            );
        self.y = self
            .y
            .clamp(
                self.size,
                height - self.size,
            );
    }
}