#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}
impl Vec2{
    pub fn new(
        x: f32,
        y: f32,
    ) -> Self {
        Self {
            x, y
        }
    }
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0
        }
    }
    pub fn length(
        &self,
    ) -> f32 {
        let squared = self.x * self.x + self.y * self.y;
        squared.sqrt()
    }
    pub fn normalized(
        &self
    ) -> Self {
        let length = self.length();
        if length == 0.0 {
            return Self::zero();
        }
        Self {
            x: self.x / length,
            y: self.y / length
        }
    }
    pub fn distance(
        &self,
        other: &Self
    ) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
    pub fn add(
        &self,
        other: &Self
    ) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
    pub fn subtract(
        &self,
        other: &Self
    ) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
    pub fn scale(
        &self,
        amount: f32
    ) -> Self {
        Self {
            x: self.x * amount,
            y: self.y * amount,
        }
    }
}