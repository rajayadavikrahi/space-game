#[test]
fn two_circles_that_overlap_shoud_collide() {
    let first_x = 0.0;
    let first_y = 0.0;
    let first_radius = 10.0;

    let second_x = 15.0;
    let second_y = 0.0;
    let second_radius = 10.0;

    let distance_squared = (first_x - second_x) * (first_x - second_x)
            + (first_y - second_y) * (fist_y - second_y);

    let radius_sum = 
        first_radius + second_radius;

    let collided = distance_squared < radius_sum * radius_sum;

    assert!(
        collided,
        "The circles should collide"
    );
}

#[test]
fn two_circles_far_apart_should_not_collide() {
    let first_x = 0.0;
    let first_y = 0.0;
    let first_radius = 10.0;

    let second_x = 100.0;
    let second_y = 100.0;
    let second_radius = 10.0;

    let dx = first_x - second_x;
    let dy = first_y - second_y;
    let distance_squared = dx * dx + dy * dy;

    let radius_sum = first_radius + second_radius;
    let collided = distance_squared < radius_sum * radius_sum;

    assert!(
        !collided,
        "The circles should not collide"
    );
}

#[test]
fn touching_circles_should_not_be_inside_each_other() {
    let first_x = 0.0;
    let first_y = 0.0;
    let first_radius = 10.0;

    let second_x = 20.0;
    let second_y = 0.0;
    let second_radius = 10.0;

    let dx = first_x - second_x;
    
    let dy = first_y - second_y;

    let distance_squared = dx * dx + dy * dy;
    let radius_sum = first_radius + second_radius;

    let collided = distance_squared < radius_sum * radius_sum;

    assert!(
        !collided,
        "Exactly touching circle should not collide with the current algo "
    )
}