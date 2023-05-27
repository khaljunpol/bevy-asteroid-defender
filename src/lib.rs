const FIRE_COOLDOWN: f64 = 0.1; // Only allow user to shoot 10 bullets/sec.
const POWERUP_COOLDOWN: f64 = 5.0;

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