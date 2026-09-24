// Import the PowerUp object.
use super::powerup::PowerUp;

// Import the object responsible
// for creating new power-ups.
use super::spawner::PowerUpSpawner;


#[derive(Debug)]
pub struct PowerUpManager {

    // `Vec<PowerUp>` means a growable list
    // containing PowerUp objects.
    //
    // Example:
    //
    // [
    //     Shield,
    //     Speed,
    //     DoubleScore
    // ]
    powerups: Vec<PowerUp>,

    // The spawner controls WHEN a new
    // power-up should appear.
    spawner: PowerUpSpawner,
}


impl PowerUpManager {

    // Create an empty manager.
    pub fn new() -> Self {
        Self {
            powerups: Vec::new(),
            spawner: PowerUpSpawner::new(),
        }
    }


    // Update the power-up system.
    //
    // This does two things:
    //
    // 1. Ask the spawner whether a new
    //    power-up should appear.
    //
    // 2. Decrease the lifetime of existing
    //    power-ups.
    pub fn update(
        &mut self,
        delta_time: f32,
        width: f32,
        height: f32,
    ) {

        // Ask the spawner to potentially
        // create a new PowerUp.
        //
        // `Option<PowerUp>` means:
        //
        // Some(powerup)
        //     A new power-up was created.
        //
        // None
        //     Nothing spawned this frame.
        let new_powerup =
            self.spawner.update(
                delta_time,
                width,
                height,
            );


        // `if let Some(powerup)` means:
        //
        // "If the Option contains a PowerUp,
        // give me that PowerUp."
        if let Some(powerup) =
            new_powerup
        {
            // Add the new power-up
            // to our Vec.
            self.powerups.push(
                powerup
            );
        }


        // Iterate through every power-up
        // currently inside the Vec.
        //
        // `&mut self.powerups`
        // gives mutable access to the Vec.
        //
        // `.iter_mut()`
        // gives mutable access to each item.
        for powerup
            in self.powerups.iter_mut()
        {
            // Decrease the item's world lifetime.
            powerup.lifetime -=
                delta_time;
        }


        // Remove power-ups whose lifetime
        // has reached zero.
        self.remove_expired();
    }


    // Give read-only access to all power-ups.
    //
    // `&self` means we only borrow the manager.
    //
    // `&[PowerUp]` is a slice.
    //
    // A slice lets callers READ the list
    // without owning the Vec.
    pub fn powerups(
        &self,
    ) -> &[PowerUp] {
        &self.powerups
    }

    // Implement Deref to allow direct iteration over powerups
    pub fn iter(&self) -> std::slice::Iter<PowerUp> {
        self.powerups.iter()
    }


    // Give mutable access to the power-ups.
    //
    // We still return a slice instead of
    // exposing the Vec itself.
    pub fn powerups_mut(
        &mut self,
    ) -> &mut [PowerUp] {
        &mut self.powerups
    }


    // Remove expired power-ups.
    pub fn remove_expired(
        &mut self,
    ) {

        // `retain` keeps only elements where
        // the closure returns true.
        //
        // So:
        //
        // lifetime > 0
        //     KEEP
        //
        // lifetime <= 0
        //     REMOVE
        self.powerups.retain(
            |powerup| {
                powerup.lifetime > 0.0
            }
        );
    }


    // Remove one power-up by its index.
    //
    // Example:
    //
    // Vec:
    //
    // [Shield, Speed, DoubleScore]
    //
    // remove(1)
    //
    // becomes:
    //
    // [Shield, DoubleScore]
    //
    // We return `Option<PowerUp>` because
    // the requested index might not exist.
    pub fn remove(
        &mut self,
        index: usize,
    ) -> Option<PowerUp> {

        // Check whether the index exists.
        if index <
            self.powerups.len()
        {
            // `.remove(index)` removes the
            // element and gives ownership
            // of that PowerUp back to us.
            Some(
                self.powerups.remove(
                    index
                )
            )
        } else {
            // Index doesn't exist.
            None
        }
    }


    // Remove every power-up and reset
    // the spawning timer.
    pub fn clear(
        &mut self,
    ) {
        self.powerups.clear();

        self.spawner.reset();
    }


    // Return the number of active
    // world power-ups.
    pub fn count(
        &self,
    ) -> usize {
        self.powerups.len()
    }

    // Check collisions between player and power-ups
    pub fn check_collisions(
        &mut self,
        player: &mut crate::game::player::Player,
        active_effects: &mut super::state::ActiveEffects,
        _score: &mut u32,
    ) {
        use crate::game::collision::circles_collide;

        let mut to_remove = Vec::new();

        for (i, powerup) in self.powerups.iter().enumerate() {
            if circles_collide(
                player.x(), player.y(), player.size,
                powerup.x, powerup.y, powerup.size,
            ) {
                // Activate the power-up effect
                active_effects.activate(powerup.kind, powerup.duration);

                // Apply immediate effects
                match powerup.kind {
                    super::kind::PowerUpKind::DoubleScore => {
                        // Score multiplier is handled in collision detection
                    }
                    _ => {}
                }

                to_remove.push(i);
            }
        }

        // Remove collected power-ups (in reverse order to maintain indices)
        for i in to_remove.into_iter().rev() {
            self.powerups.remove(i);
        }
    }
}
