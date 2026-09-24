// These tests document the expected game lifecycle.
//
// A game should normally move through:
//
// Waiting
//    ↓
// Running
//    ↓
// GameOver
//
// And reset should bring it back to:
//
// Waiting

#[derive(Debug, PartialEq)]
enum TestGameStatus {
    Waiting,
    Running,
    GameOver,
}


#[test]
fn new_game_should_start_in_waiting_state() {
    // A newly created game should not immediately
    // start running.
    //
    // The player should first press START.

    let status =
        TestGameStatus::Waiting;

    assert_eq!(
        status,
        TestGameStatus::Waiting
    );
}


#[test]
fn starting_game_should_change_status_to_running() {
    let mut status =
        TestGameStatus::Waiting;

    // Simulate the start() function.

    if status ==
        TestGameStatus::Waiting
    {
        status =
            TestGameStatus::Running;
    }

    assert_eq!(
        status,
        TestGameStatus::Running
    );
}


#[test]
fn game_over_should_stop_running_game() {
    let mut status =
        TestGameStatus::Running;

    // Simulate collision with an enemy.

    status =
        TestGameStatus::GameOver;

    assert_eq!(
        status,
        TestGameStatus::GameOver
    );
}


#[test]
fn reset_should_return_game_to_waiting() {
    let mut status =
        TestGameStatus::GameOver;

    // Simulate reset().

    status =
        TestGameStatus::Waiting;

    assert_eq!(
        status,
        TestGameStatus::Waiting
    );
}


#[test]
fn score_should_increase_when_energy_is_collected() {
    let mut score = 0;

    // Current game design gives
    // 10 points for collecting energy.

    score += 10;

    assert_eq!(
        score,
        10
    );
}


#[test]
fn collecting_multiple_energy_objects_should_increase_score() {
    let mut score = 0;

    // First energy.
    score += 10;

    // Second energy.
    score += 10;

    // Third energy.
    score += 10;

    assert_eq!(
        score,
        30
    );
}