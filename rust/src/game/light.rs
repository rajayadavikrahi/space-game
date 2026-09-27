// ============================================
// THE LAMP
// ============================================
//
// The grid is down. The only light left in the
// arena is the lamp the player carries, so the
// lamp is the whole game:
//
// * it burns battery while it is on,
// * it is the only way to see the energy orbs,
// * it dims as the battery empties,
// * and it pulls the enemies toward you.
//
// This struct is the single source of truth for
// all of that. The browser only draws what this
// module says.

/// Full battery, in percent.
pub const MAX_BATTERY: f32 = 100.0;

/// Battery used up every second while burning.
pub const DRAIN_PER_SECOND: f32 = 5.0;

/// Battery regained every second while the lamp
/// is switched off. Recharging is as slow as
/// draining, so every second of light is a second
/// the player pays for later.
pub const RECHARGE_PER_SECOND: f32 = 5.0;

/// Battery gained by collecting one energy orb.
pub const ENERGY_CHARGE: f32 = 25.0;

/// Radius of the light when the battery is full.
pub const MAX_RADIUS: f32 = 270.0;

/// Radius of the light when the battery is empty.
/// The lamp never shrinks to nothing, it just
/// turns into a candle.
pub const MIN_RADIUS: f32 = 90.0;

/// How much faster enemies move inside the
/// beam. Light gives you sight, it also gives
/// away your position.
pub const ENEMY_LURE_MULTIPLIER: f32 = 1.6;

#[derive(Debug)]
pub struct Light {
    on: bool,
    battery: f32,
}

impl Light {
    // Create a lamp that is switched on and fully
    // charged.
    pub fn new() -> Self {
        Self {
            on: true,
            battery: MAX_BATTERY,
        }
    }

    // Did the player leave the lamp switched on?
    //
    // This is the switch position, it can be true
    // even when the battery is dead.
    pub fn is_on(&self) -> bool {
        self.on
    }

    // Is the lamp actually producing light right
    // now? A lamp with a flat battery is off no
    // matter where the switch is.
    pub fn is_lit(&self) -> bool {
        self.on && self.battery > 0.0
    }

    // Flip the switch.
    pub fn toggle(&mut self) {
        self.set_on(!self.on);
    }

    // Put the switch in the requested position.
    //
    // A flat battery cannot be lit, so turning the
    // lamp on while the battery is empty does
    // nothing. The lamp stays dead until the
    // battery has recharged.
    pub fn set_on(&mut self, on: bool) {
        if on && self.battery <= 0.0 {
            self.on = false;
            return;
        }

        self.on = on;
    }

    pub fn battery(&self) -> f32 {
        self.battery
    }

    // Battery as a 0.0 to 1.0 fraction, which is
    // what the browser draws its battery bar from.
    pub fn battery_percent(&self) -> f32 {
        (self.battery / MAX_BATTERY).clamp(0.0, 1.0)
    }

    // How far the light reaches, in pixels.
    //
    // A dying battery shrinks the beam, so the
    // warning is in the picture before it is in
    // the HUD. Returns 0.0 while the lamp is dead.
    pub fn radius(&self) -> f32 {
        if !self.is_lit() {
            return 0.0;
        }

        MIN_RADIUS
            + (MAX_RADIUS - MIN_RADIUS) * self.battery_percent()
    }

    // How fast an enemy `distance` pixels away
    // should move.
    //
    // Enemies inside the beam hurry toward the
    // player. Outside it they keep their normal
    // speed, so switching the lamp off really does
    // slow the hunt down.
    pub fn enemy_lure_multiplier(
        &self,
        distance: f32,
    ) -> f32 {
        if !self.is_lit() {
            return 1.0;
        }

        if distance <= self.radius() {
            ENEMY_LURE_MULTIPLIER
        } else {
            1.0
        }
    }

    // Put some charge into the battery, for
    // example when an energy orb is collected.
    pub fn charge(
        &mut self,
        amount: f32,
    ) {
        self.battery =
            (self.battery + amount).min(MAX_BATTERY);
    }

    // Fill the battery and light the lamp. This
    // is what a lantern power-up does.
    pub fn recharge_full(&mut self) {
        self.battery = MAX_BATTERY;
        self.on = true;
    }

    // Burn or recover battery for one tick.
    //
    // Burning costs battery and, when the battery
    // runs out, the lamp dies on its own. A dead
    // lamp recharges while it is off, so the dark
    // is the only way to get light back.
    pub fn update(
        &mut self,
        delta_time: f32,
    ) {
        if self.is_lit() {
            self.battery -= DRAIN_PER_SECOND * delta_time;

            if self.battery <= 0.0 {
                self.battery = 0.0;
                self.on = false;
            }
        } else {
            self.battery =
                (self.battery + RECHARGE_PER_SECOND * delta_time)
                    .min(MAX_BATTERY);
        }
    }

    // Start every new game with a working lamp.
    pub fn reset(&mut self) {
        self.on = true;
        self.battery = MAX_BATTERY;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_lamp_starts_on_and_charged() {
        let light = Light::new();

        assert!(light.is_on());
        assert!(light.is_lit());
        assert_eq!(light.battery(), MAX_BATTERY);
    }

    #[test]
    fn toggling_turns_the_lamp_off() {
        let mut light = Light::new();

        light.toggle();

        assert!(!light.is_on());
        assert!(!light.is_lit());
    }

    #[test]
    fn burning_drains_the_battery() {
        let mut light = Light::new();

        light.update(2.0);

        assert_eq!(
            light.battery(),
            MAX_BATTERY - DRAIN_PER_SECOND * 2.0
        );
    }

    #[test]
    fn a_flat_battery_kills_the_lamp() {
        let mut light = Light::new();

        // Burn for longer than the battery lasts.
        light.update(MAX_BATTERY / DRAIN_PER_SECOND + 1.0);

        assert_eq!(light.battery(), 0.0);
        assert!(!light.is_on());
        assert!(!light.is_lit());
    }

    #[test]
    fn a_dead_lamp_recharges_while_it_is_off() {
        let mut light = Light::new();

        light.update(MAX_BATTERY / DRAIN_PER_SECOND + 1.0);
        light.update(4.0);

        assert_eq!(light.battery(), RECHARGE_PER_SECOND * 4.0);
    }

    #[test]
    fn a_dead_lamp_cannot_be_switched_on() {
        let mut light = Light::new();

        light.update(MAX_BATTERY / DRAIN_PER_SECOND + 1.0);
        light.set_on(true);

        assert!(!light.is_on());
        assert!(!light.is_lit());
    }

    #[test]
    fn the_battery_never_overfills() {
        let mut light = Light::new();

        light.charge(MAX_BATTERY * 2.0);

        assert_eq!(light.battery(), MAX_BATTERY);
        assert_eq!(light.battery_percent(), 1.0);
    }

    #[test]
    fn the_beam_shrinks_as_the_battery_drains() {
        let mut light = Light::new();

        let full = light.radius();

        light.update(10.0);

        let half = light.radius();

        assert_eq!(full, MAX_RADIUS);
        assert!(half < full);
        assert!(half >= MIN_RADIUS);
    }

    #[test]
    fn a_dead_lamp_has_no_reach() {
        let mut light = Light::new();

        light.update(MAX_BATTERY / DRAIN_PER_SECOND + 1.0);

        assert_eq!(light.radius(), 0.0);
    }

    #[test]
    fn enemies_inside_the_beam_move_faster() {
        let light = Light::new();

        assert_eq!(
            light.enemy_lure_multiplier(10.0),
            ENEMY_LURE_MULTIPLIER
        );

        assert_eq!(
            light.enemy_lure_multiplier(light.radius() + 1.0),
            1.0
        );
    }

    #[test]
    fn a_dark_lamp_lures_nothing() {
        let mut light = Light::new();

        light.toggle();

        assert_eq!(light.enemy_lure_multiplier(0.0), 1.0);
    }

    #[test]
    fn a_lantern_refills_and_relights_the_lamp() {
        let mut light = Light::new();

        light.update(MAX_BATTERY / DRAIN_PER_SECOND + 1.0);
        light.recharge_full();

        assert!(light.is_lit());
        assert_eq!(light.battery(), MAX_BATTERY);
    }
}
