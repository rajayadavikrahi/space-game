#[derive(Debug, Clone, Copy)]
pub enum PowerUpKind{
    Shield,
    Speed,
    DoubleScore,

    // Fills the lamp battery and switches the
    // lamp back on. It is the only power-up that
    // hands out light instead of a stat boost.
    Lantern,
}
