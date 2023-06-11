use bevy::prelude::*;

pub fn get_angle_to_target(target: Vec2, origin: Vec2) -> Vec2 {
    let direction = target - origin;
    let rotation = direction.normalize() * 1.0;

    rotation
}

pub fn calculate_max_spawn_distance(angle: f32, window_size: Vec2) -> f32 {
    let half_window_height = window_size.y / 2.0;
    let angle_ratio = angle.abs() / 90.0;
    
    let max_distance = half_window_height * angle_ratio;
    
    max_distance
}