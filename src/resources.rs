use bevy::prelude::*;

pub const PLAYER_SPRITE: &str = "sprites/ships/playerShip2_blue.png";

pub const SHIPS_SPRITE: [&str; 12] = [
    "playerShip1_blue.png",
    "playerShip1_green.png",
    "playerShip1_orange.png",
    "playerShip1_red.png",
    "playerShip2_blue.png",
    "playerShip2_green.png",
    "playerShip2_orange.png",
    "playerShip2_red.png",
    "playerShip3_blue.png",
    "playerShip3_green.png",
    "playerShip3_orange.png",
    "playerShip3_red.png",
];

pub struct GameSprites {
    pub player: Handle<Image>,
    pub asteroid_xl: Handle<Image>,
    pub asteroid_m: Handle<Image>,
    pub asteroid_s: Handle<Image>,
}

#[derive(Resource)]
pub struct WindowSize {
    pub w: f32,
    pub h: f32,
}
