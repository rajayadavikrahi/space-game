use super::vector::Vec2;

#[derive(Debug)]
pub struct Player {
    pub position: Vec2,
    pub size: f32,
    pub speed: f32
}

impl Player {
    pub fn new(
        x: f32,
        y: f32 
    ) -> Self {
        Self {
            position:
                Vec2::new(
                    x, y
                ),
                size: 20.0,
                speed: 300.0
        }
    }
    pub fn x(
        &self
    ) -> f32 {
        self.position.x
    }
    pub fn y(
        &self
    ) -> f32 {
        self.position.y
    }
    pub fn move_by(
        &mut self,
        direction_x: f32,
        direction_y: f32,
        delta_time: f32
    ) {
        self.move_by_with_speed(
            direction_x,
            direction_y,
            delta_time,
            1.0
        );
    }
    pub fn move_by_with_speed(
        &mut self,
        direction_x: f32,
        direction_y: f32,
        delta_time: f32,
        speed_multiplier: f32
    ) {
        let direction = Vec2::new(
            direction_x,
            direction_y
        );
        let direction = direction.normalized();
        let movement = direction.scale(
            self.speed * speed_multiplier * delta_time
        );
        self.position = self.position.add(
            &movement
        );
    }
    pub fn keep_inside(
        &mut self,
        width: f32,
        height: f32
    ) {
        self.position.x = self.position.x.clamp(self.size, width - self.size);
        self.position.y = self.position.y.clamp(self.size, height - self.size);
    }

    pub fn update(
        &mut self,
        direction_x: f32,
        direction_y: f32,
        dt: f32,
        width: f32,
        height: f32
    ) {
        self.move_by(direction_x, direction_y, dt);
        self.keep_inside(width, height);
    }
}