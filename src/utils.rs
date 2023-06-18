use bevy::prelude::*;

pub fn get_angle_to_target(target: Vec2, origin: Vec2) -> Vec2 {
    let direction = target - origin;
    let rotation = direction.normalize() * 1.0;

    rotation
}

pub fn calculate_max_spawn_distance(window_size: Vec2) -> f32 {
    let half_window_height = window_size.x / 2.0;
    
    half_window_height
}