#[derive(Debug)]
pub struct Energy {
    pub x: f32, 
    pub y: f32, 
    pub size: f32
}

impl Energy {
    pub fn new(
        x: f32,
        y: f32,
        size: f32,
    ) -> Self {
        Self {
            x, y, size,
        }
    }
    pub fn move_to(
        &mut self,
        x: f32,
        y: f32
    ) {
        self.x = x;
        self.y = y;
    }
}