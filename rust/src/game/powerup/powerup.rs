// Import the enum that tells us what type
// of power-up this object represents.
use super::kind::PowerUpKind;


// This struct represents a power-up item
// that physically exists inside the game world.
//
// Example:
//
// A green Shield item sitting at:
//
// x = 300
// y = 200
//
// is represented by one PowerUp struct.
#[derive(Debug)]
pub struct PowerUp {

    // X coordinate of the power-up.
    pub x: f32,

    // Y coordinate of the power-up.
    pub y: f32,

    // Radius/size used for drawing
    // and collision detection.
    pub size: f32,

    // Which kind of power-up this is.
    //
    // Example:
    //
    // Shield
    // Speed
    // DoubleScore
    pub kind: PowerUpKind,

    // How long the EFFECT should last
    // after the player collects it.
    //
    // IMPORTANT:
    //
    // This is no longer the lifetime of
    // the item sitting on the map.
    pub duration: f32,

    // How long the physical power-up
    // remains in the world before disappearing.
    //
    // This is separate from `duration`.
    pub lifetime: f32,
}


impl PowerUp {

    // Constructor for creating a new PowerUp.
    //
    // `Self` means the type currently being
    // implemented, which is `PowerUp`.
    pub fn new(
        x: f32,
        y: f32,
        kind: PowerUpKind,
        duration: f32,
    ) -> Self {

        Self {
            x,
            y,

            // All power-ups use radius 10.
            size: 10.0,

            kind,

            // Store how long the collected
            // effect should remain active.
            duration,

            // The physical item remains on the
            // map for 20 seconds.
            lifetime: 20.0,
        }
    }


    // Move the power-up to another position.
    //
    // `&mut self` means:
    //
    // "Borrow this PowerUp mutably so
    // I can modify it."
    pub fn move_to(
        &mut self,
        x: f32,
        y: f32,
    ) {
        self.x = x;
        self.y = y;
    }
}