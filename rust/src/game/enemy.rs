// Import two-dimensional vector type.
use super::vector::Vec2;


// Store enemy state.
#[derive(Debug)]
pub struct Enemy {

    // Store enemy position.
    pub position: Vec2,

    // Store enemy collision size.
    pub size: f32,

    // Store enemy movement speed.
    pub speed: f32,
}


impl Enemy {

    // Create a new enemy.
    pub fn new(
        x: f32,
        y: f32,
        speed: f32,
    ) -> Self {
        Self {

            // Create enemy position.
            position:
                Vec2::new(
                    x,
                    y,
                ),

            // Set enemy radius.
            size: 18.0,

            // Store movement speed.
            speed,
        }
    }


    // Return enemy X coordinate.
    pub fn x(
        &self,
    ) -> f32 {
        self.position.x
    }


    // Return enemy Y coordinate.
    pub fn y(
        &self,
    ) -> f32 {
        self.position.y
    }


    // Move enemy toward target.
    pub fn move_towards(
        &mut self,
        target_x: f32,
        target_y: f32,
        delta_time: f32,
    ) {

        // Create target position.
        let target =
            Vec2::new(
                target_x,
                target_y,
            );


        // Calculate direction to target.
        let direction =
            target.subtract(
                &self.position
            );


        // Normalize movement direction.
        let direction =
            direction.normalized();


        // Calculate movement distance.
        let movement =
            direction.scale(
                self.speed
                    * delta_time,
            );


        // Move enemy toward player.
        self.position =
            self.position.add(
                &movement
            );
    }

    pub fn update(
        &mut self,
        player: &crate::game::player::Player,
        dt: f32,
    ) {
        self.move_towards(player.x(), player.y(), dt);
    }
}