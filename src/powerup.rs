use bevy::prelude::*;
use rand::{
    prelude::*
};
use lib::{PowerUpType, ShipType};

use crate::{
    common_components::{RotationAngle},
    resources::{GameSprites}
};

#[derive(Component)]
pub struct PowerUpComponent {
    pub rotation_speed: f32,
    pub powerup_type: PowerUpType,

    change_target: ShipType,
    isSpriteUpdated: bool
}

impl PowerUpComponent {
    pub fn new(rotation_speed: f32) -> PowerUpComponent {
        let powerup_type = Self::randomize_type();
        let mut rng = thread_rng();
        let change_target = match rng.gen_range(0..3) {
            0 => ShipType::Attack,
            1 => ShipType::Shield,
            _ => ShipType::Normal,
        };

        PowerUpComponent { powerup_type, 
            rotation_speed, change_target, 
            isSpriteUpdated: false
         }
    }

    pub fn get_ship_change_type(&self) -> ShipType {
        return self.change_target;
    }

    fn randomize_type() -> PowerUpType {
        let mut rng = thread_rng();
        // Generate a random Powerup Type
        rng.gen()
    }
}

pub struct PowerUpPlugin;

impl Plugin for PowerUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(powerup_rotation_system)
        .add_system(powerup_change_sprite_system);
    }
}

fn powerup_rotation_system(mut query: Query<(&PowerUpComponent, &mut RotationAngle)>) {
    for (powerup, mut rotation_angle) in query.iter_mut() {
        rotation_angle.0 += powerup.rotation_speed;
    }
}

fn powerup_change_sprite_system(
    game_sprites: Res<GameSprites>,
    mut query: Query<(&mut Handle<Image>, &mut PowerUpComponent), With<PowerUpComponent>>,
)
{
    for (mut texture_handle, mut powerup) in query.iter_mut() {
        if !powerup.isSpriteUpdated {
            // Load a new texture and update the handle
            let new_texture_handle: Handle<Image> = match powerup.powerup_type {
                PowerUpType::ChangeShipType => {
                    match powerup.change_target {
                        ShipType::Attack => game_sprites.powerup_change_attack.clone(),
                        ShipType::Shield => game_sprites.powerup_change_shield.clone(),
                        ShipType::Normal => game_sprites.powerup_change_normal.clone(),
                    }
                }
            };

            *texture_handle = new_texture_handle;

            powerup.isSpriteUpdated = true;
        }
    }
}