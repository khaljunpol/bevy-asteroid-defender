use bevy::prelude::*;
use lib::{PROJECTILE_DESPAWN_TIME, ShipType, PROJECTILE_SIZE, PROJECTILE_SPEED};

use crate::{
    resources::GameSprites, 
    player::PlayerComponent, 
    common_components::{RotationAngle, Position, HitBoxSize, Velocity}, 
    ship::ShipComponent
};

#[derive(Component)]
pub struct ProjectileComponent;

#[derive(Component)]
pub struct ProjectileDespawnComponent (pub Timer);

impl Default for ProjectileDespawnComponent {
    fn default() -> Self {
        Self(Timer::from_seconds(PROJECTILE_DESPAWN_TIME, TimerMode::Once))
    }
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(projectile_despawn_system)
        .add_system(projectile_shoot_system);
    }
}

fn projectile_despawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ProjectileDespawnComponent)>,
) {
    for (entity, mut despawn_timer) in query.iter_mut() {
        despawn_timer.0.tick(time.delta());

        if despawn_timer.0.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn projectile_shoot_system(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    game_sprites: Res<GameSprites>,
    mut ship_query: Query<(&ShipComponent), With<PlayerComponent>>,
    mut query: Query<(&PlayerComponent, &RotationAngle, &Position)>,
) {
    
    for (player, rotation_angle, position) in query.iter_mut() {
        if let Ok(ship) = ship_query.get_single_mut() {

            let laser_texture_handle: Handle<Image> = match ship.ship_type {
                ShipType::Attack => game_sprites.projectile_attack.clone(),
                ShipType::Shield => game_sprites.projectile_shield.clone(),
                ShipType::Normal => game_sprites.projectile_normal.clone(),
            };

            if kb.pressed(KeyCode::Space) {
                commands
                    .spawn(SpriteBundle {
                        texture: laser_texture_handle,
                        transform: Transform {
                            translation: Vec3::new(position.0.x, position.0.y, 5.),
                            scale: Vec3::new(0.5, 0.5, 1.),
                            rotation: Quat::from_rotation_z(rotation_angle.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Name::new("Player laser"))
                    .insert(ProjectileComponent)
                    .insert(ProjectileDespawnComponent::default())
                    .insert(HitBoxSize(PROJECTILE_SIZE))
                    .insert(Velocity(
                        player.direction(rotation_angle.0).normalize() * PROJECTILE_SPEED,
                    ))
                    .insert(Position(position.0.clone()));
            }
        }
    }
}
