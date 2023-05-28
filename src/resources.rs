use bevy::prelude::*;

pub const SHIP_NORMAL_SPRITE: &str = "sprites/ships/playerShip1_blue.png";
pub const SHIP_ATTACK_SPRITE: &str = "sprites/ships/playerShip1_red.png";
pub const SHIP_SHIELD_SPRITE: &str = "sprites/ships/playerShip1_green.png";

pub const SHIPS_SPRITE: [&str; 9] = [
    "playerShip1_blue.png",
    "playerShip1_green.png",
    "playerShip1_red.png",
    "playerShip2_blue.png",
    "playerShip2_green.png",
    "playerShip2_red.png",
    "playerShip3_blue.png",
    "playerShip3_green.png",
    "playerShip3_red.png",
];

#[derive(Resource)]
pub struct GameSprites {
    pub ship_type_normal: Handle<Image>,
    pub ship_type_attack: Handle<Image>,
    pub ship_type_shield: Handle<Image>,
}

#[derive(Resource)]
pub struct WindowSize {
    pub w: f32,
    pub h: f32,
}