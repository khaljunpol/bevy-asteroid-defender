use bevy::prelude::*;

pub const SHIP_NORMAL_SPRITE: &str = "sprites/ships/playerShip1_blue.png";
pub const SHIP_ATTACK_SPRITE: &str = "sprites/ships/playerShip1_red.png";
pub const SHIP_SHIELD_SPRITE: &str = "sprites/ships/playerShip1_green.png";

pub const POWERUP_CHANGE_NORMAL_SPRITE: &str = "sprites/powerup/powerupBlue_star.png";
pub const POWERUP_CHANGE_ATTACK_SPRITE: &str = "sprites/powerup/powerupRed_star.png";
pub const POWERUP_CHANGE_SHIELD_SPRITE: &str = "sprites/powerup/powerupGreen_star.png";

pub const PROJECTILE_NORMAL_SPRITE: &str = "sprites/laser/laserBlue01.png";
pub const PROJECTILE_ATTACK_SPRITE: &str = "sprites/laser/laserRed01.png";
pub const PROJECTILE_SHIELD_SPRITE: &str = "sprites/laser/laserGreen01.png";

#[derive(Resource )]
pub struct GameSprites {
    pub ship_type_normal: Handle<Image>,
    pub ship_type_attack: Handle<Image>,
    pub ship_type_shield: Handle<Image>,
    pub powerup_change_normal: Handle<Image>,
    pub powerup_change_attack: Handle<Image>,
    pub powerup_change_shield: Handle<Image>,
    pub projectile_normal: Handle<Image>,
    pub projectile_attack: Handle<Image>,
    pub projectile_shield: Handle<Image>
}

#[derive(Resource)]
pub struct WindowSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Resource)]
pub struct WindowDespawnBorder {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}