pub fn circles_collide (
    first_x: f32,
    first_y: f32,
    first_radius: f32,
    second_x: f32,
    second_y: f32,
    second_radius: f32
) -> bool { 
    let dx = first_x - second_x;
    let dy = first_y - second_y;

    let distance_squared = dx * dx + dy * dy;

    let radius_sum = first_radius + second_radus;

    distance_squared < radius_sum * radius_sum
}