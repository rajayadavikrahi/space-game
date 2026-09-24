use void_runner::game::powerup::powerup::PowerUp;
use void_runner::game::powerup::spawner::PowerUpSpawner;

#[test]
fn creates_shield_powerup() {
    let power_up = PowerUp::new(
        100.0,
        200.0,
        PowerUpKind::Shield,
        5.0
    );

    assert_eq!(
        power_up.x,
        100.0
    );

    assert_eq!(
        power_up.y,
        200
    );

    assert_eq!(
        power_up.duration,
        5.0
    );

    assert!(
        matches!(
            power_up.kind,
            PowerUpKind::Sheild
        )
    );
}

#[test]
fn creates_speed_powerup() {
    let power_up = PowerUp::new(
        300.0,
        400.0,
        PowerUpKind::Speed,
        5.0
    );
    assert!(
        matches!(
            power_up.kind,
            PowerUpKind::Speed
        )
    );
}

#[test]
fn creates_double_score_powerup() {
    let power_up = PowerUp::new(
        500.0,
        100.0,
        PowerUpKind::DoubleScore,
        5.0
    );
    assert!(
        matches!(
            power_up.kind,
            PowerUpKind::DoubleScore
        )
    );
}

#[test]
fn moves_powerup() {
    let mut power_up = PowerUp::new(
        100.0,
        100.0,
        PowerUpKind::Shield,
        5.0
    );
    power_up.move_to(
        500.0,
        300.0
    );
    assert_eq!(
        power_up.x,
        500.0
    );
    assert_eq!(
        power_up.y,
        300.0
    );
}

#[test]
fn create_powerup_spawner() {
    let spawner = PowerUpSpawner::new();
    assert_eq!(
        spawner.interval(),
        10.0
    );
}

#[test]
fn spawner_does_not_spawn_immediately() {
    let mut spawner = PowerUpSpawner::new();
    let result = spawner.update(
        1.0, 900.0, 600.0
    );
    assert!(result.is_none())
}


#[test]
fn spawner_creates_powerup() {
    let mut spawner = PowerUpSpawner::new();
    let result = spawner.update(
        10.0,
        900.0,
        600.0
    );
    assert!(
        result.is_some()
    )
}
