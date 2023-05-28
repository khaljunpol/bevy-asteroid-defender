use std::f32::consts::PI;

pub const PLAYER_SIZE: (f32, f32) = (112., 75.);

pub const PLAYER_TURN_SPEED: f32 = 5.0 * 2.0 * PI / 360.0;
pub const PLAYER_ACCELERATION: f32 = 0.2;
pub const PLAYER_DECELERATION: f32 = 0.01;
pub const PLAYER_MAX_SPEED: f32 = 7.0;
pub const SPRITE_SCALE: f32 = 0.5;