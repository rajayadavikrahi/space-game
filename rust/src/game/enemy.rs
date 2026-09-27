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


    // Move enemy toward target using a speed
    // multiplier.
    //
    // The lamplight uses this: an enemy standing
    // inside the beam moves faster than one hiding
    // in the dark.
    pub fn move_towards(
        &mut self,
        target_x: f32,
        target_y: f32,
        delta_time: f32,
        speed_multiplier: f32,
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
                    * speed_multiplier
                    * delta_time,
            );


        // Move enemy toward player.
        self.position =
            self.position.add(
                &movement
            );
    }

    // Chase the player at the given speed.
    pub fn update(
        &mut self,
        player: &crate::game::player::Player,
        dt: f32,
        speed_multiplier: f32,
    ) {
        self.move_towards(
            player.x(),
            player.y(),
            dt,
            speed_multiplier,
        );
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::player::Player;

    // The lamplight hands the world a speed
    // multiplier, so the enemy has to honour it.
    #[test]
    fn a_lured_enemy_closes_faster() {
        let player = Player::new(100.0, 100.0);

        let mut calm = Enemy::new(0.0, 100.0, 100.0);
        let mut lured = Enemy::new(0.0, 100.0, 100.0);

        calm.update(&player, 0.1, 1.0);
        lured.update(&player, 0.1, 1.6);

        assert!(
            lured.x() > calm.x(),
            "lured reached {}, calm reached {}",
            lured.x(),
            calm.x()
        );
    }

    #[test]
    fn an_enemy_ignores_a_multiplier_of_one() {
        let player = Player::new(100.0, 100.0);

        let mut a = Enemy::new(0.0, 100.0, 100.0);
        let mut b = Enemy::new(0.0, 100.0, 100.0);

        a.update(&player, 0.1, 1.0);
        b.update(&player, 0.1, 1.0);

        assert_eq!(a.x(), b.x());
    }
}
