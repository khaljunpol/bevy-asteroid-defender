use bevy::prelude::*;

pub const ships: [String; 12] = [
    "playerShip1_blue.png".to_string(),
    "playerShip1_green.png".to_string(),
    "playerShip1_orange.png".to_string(),
    "playerShip1_red.png".to_string(),
    "playerShip2_blue.png".to_string(),
    "playerShip2_green.png".to_string(),
    "playerShip2_orange.png".to_string(),
    "playerShip2_red.png".to_string(),
    "playerShip3_blue.png".to_string(),
    "playerShip3_green.png".to_string(),
    "playerShip3_orange.png".to_string(),
    "playerShip3_red.png".to_string()
];

pub struct GameSprites {
    pub player: Handle<Image>,
    pub asteroid_xl: Handle<Image>,
    pub asteroid_m: Handle<Image>,
    pub asteroid_s: Handle<Image>,
}