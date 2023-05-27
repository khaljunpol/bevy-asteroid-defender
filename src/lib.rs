use std::f32::consts::PI;

pub const PLAYER_SIZE: (f32, f32) = (112., 75.);

pub const PLAYER_TURN_SPEED: f32 = 5.0 * 2.0 * PI / 360.0;
pub const PLAYER_ACCELERATION: f32 = 0.2;
pub const PLAYER_DECELERATION: f32 = 0.01;
pub const PLAYER_MAX_SPEED: f32 = 7.0;
pub const SPRITE_SCALE: f32 = 0.5;

pub enum GameStatus {
    // Normal fighting mode
    Normal,
    // Player died
    Died,
    // Player won!
    Win,
}

struct GameState {
    debug_mode: bool,
    // Overall game state
    game_status: GameStatus,    
    powerup_cooldown: f64,
    // User shooting state
    fire_bullets: bool,
    fire_cooldown: f64,
}