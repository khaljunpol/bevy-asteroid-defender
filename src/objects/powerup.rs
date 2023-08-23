use bevy::prelude::*;
use rand::{
    prelude::*
};
use lib::{PowerUpType, ShipType, POWERUP_MAX_COUNT, POWER_UP_SIZE};

use crate::{
    common_components::{RotationAngle, Velocity, Position, BoundsDespawnable, HitBoxSize},
    resources::{GameSprites, WindowSize}
};

#[derive(Component)]
pub struct PowerUpComponent {
    pub rotation_speed: f32,
    pub powerup_type: PowerUpType,

    change_target: ShipType,
    is_sprite_updated: bool
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
            is_sprite_updated: false
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
        app.add_systems(Update, (powerup_rotation_system, powerup_change_sprite_system));
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
        if !powerup.is_sprite_updated {
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

            powerup.is_sprite_updated = true;
        }
    }
}

pub fn spawn_powerup_system(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    wdw_size: Res<WindowSize>,
    query: Query<With<PowerUpComponent>>,
)
{
    let mut count = 0;
    for _ in query.iter() {
        count += 1;
    }
    
    let center = Vec2::new(wdw_size.w / 2.0, wdw_size.h / 2.0);
    let mut rng = thread_rng();

    if count < POWERUP_MAX_COUNT {
        let x_pos_rand = rng.gen_range(-center.x..center.x);
        let y_pos_rand = if rng.gen_bool(0.5) { -1 } else { 1 } as f32;

    
        let position = Vec2::new(
            x_pos_rand,  
            y_pos_rand * (center.y + 50.0));
    
        // randomizing the starting rotation angle of the powerups
        let rotation = rng.gen_range(-0.1..0.1) as f32;
    
        // randomizing rotation speed
        let rot_speed =
            rng.gen_range(-0.1..0.1) as f32;

        // randomizing movement speed
        let x_speed = rng.gen_range(-1.5..1.5);
        let mut y_speed = 0.0;

        if position.y > center.y{
            y_speed = rng.gen_range(-1.5..-1.0);
        } else if position.y < center.y{
            y_speed = rng.gen_range(1.0..1.5);
        }

        let speed = Vec2::new(x_speed, y_speed);
    
        let powerup_position = Vec3::new(position.x, position.y, 1.0);
    
        commands
            .spawn(SpriteBundle {
                texture: game_sprites.powerup_change_normal.clone(),
                transform: Transform {
                    translation: powerup_position,
                    rotation: Quat::from_rotation_z(rotation),
                    scale: Vec3::new(1.0, 1.0 ,1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Name::new("Power Up"))
            .insert(PowerUpComponent::new(rot_speed))
            .insert(HitBoxSize(POWER_UP_SIZE))
            .insert(Velocity(Vec2::from(speed)))
            .insert(Position(Vec2::new(position.x, position.y)))
            .insert(RotationAngle(rotation))
            .insert(BoundsDespawnable(Vec2::new(10.0, 10.0)));
    }
}