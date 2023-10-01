use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::prelude::*;

use lib::{METEOR_MAX_COUNT, MeteorSizeType, METEOR_DMG, METEOR_SIZE, METEOR_SPAWN_TIME};
use crate::{
    common::common_components::{RotationAngle, Velocity, Position, HitBoxSize, CollisionDespawnableWithDamage, BoundsDespawnableWithTimer, MeteorCollision},
    resources::{GameSprites, WindowSize}, 
    utils::{utils::{
            get_angle_to_target, calculate_max_spawn_distance
        }, cleanup::{CleanUpEndGame}}, 
    player::player::PlayerComponent, state::states::GameStates
};

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct MeteorComponent{
    pub size: MeteorSizeType,
    pub rotation_speed: f32
}

pub struct MeteorPlugin;

impl Plugin for MeteorPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, meteor_rotation_system)
            // Spawn
            .add_systems(Update, spawn_meteor_system
                .run_if(in_state(GameStates::InGame)
                .and_then(on_timer(Duration::from_secs_f32(METEOR_SPAWN_TIME)))));
    }
}

fn meteor_rotation_system(mut query: Query<(&MeteorComponent, &mut RotationAngle)>) {
    for (meteor, mut rotation_angle) in query.iter_mut() {
        rotation_angle.0 += meteor.rotation_speed;
    }
}

fn spawn_meteor_system(
    mut commands: Commands,
    mut game_sprites: Res<GameSprites>,
    wdw_size: Res<WindowSize>,
    query: Query<With<MeteorComponent>>,
    player_query: Query<(&Position, With<PlayerComponent>)>,
)
{
    let (player_position, _player) = player_query.single();

    let mut count = 0;
    for _ in query.iter() {
        count += 1;
    }
    
    let mut rng = thread_rng();
    let angle_offset_range = 0.0..359.0 as f32;

    let spawn_angle_list: [f32; 4] = [
        0. + rng.gen_range(angle_offset_range.clone()),
        90. + rng.gen_range(angle_offset_range.clone()),
        180. + rng.gen_range(angle_offset_range.clone()),
        270. + rng.gen_range(angle_offset_range.clone())
    ];

    if count < METEOR_MAX_COUNT {
        let angle_range = 0..spawn_angle_list.len();
        let angle_idx = rng.gen_range(angle_range.clone());
        let angle = spawn_angle_list[angle_idx];
        let max_dist = calculate_max_spawn_distance(Vec2 { x: wdw_size.w, y: wdw_size.h });

        let (x_pos_rand, y_pos_rand) = angle.to_radians().sin_cos();
    
        let position = Vec2::new(x_pos_rand * max_dist, y_pos_rand * max_dist);
    
        // randomizing the starting rotation angle of the powerups
        let rotation = rng.gen_range(-0.05..0.05) as f32;
    
        // randomizing rotation speed
        let rot_speed =
            rng.gen_range(-0.05..0.05) as f32;

        let rot_velocity = get_angle_to_target(player_position.0, position);

        let powerup_position = Vec3::new(position.x, position.y, 1.0);

        spawn_meteor(
            &mut commands,
            &mut game_sprites,
            MeteorSizeType::Large,
            powerup_position,
            position,
            rotation,
            rot_speed,
            rot_velocity,
            Vec2::new(max_dist, max_dist)
        );
    }
}

pub fn spawn_meteor(
    commands: &mut Commands,
    game_sprites: &mut Res<GameSprites>,
    size: MeteorSizeType,
    spawn_position: Vec3,
    position: Vec2,
    rotation: f32,
    rotation_speed: f32,
    velocity: Vec2,
    bounds_offset: Vec2
) {
    let (name, sprite) = match size {
        MeteorSizeType::Large => ("Big Meteor", game_sprites.meteor_big.clone()),
        MeteorSizeType::Medium => ("Med Meteor", game_sprites.meteor_med.clone()),
        MeteorSizeType::Small => ("Sml Meteor", game_sprites.meteor_sml.clone()),
        _ => ("Big Meteor", game_sprites.meteor_big.clone()),
    };

    let _ = &commands
    .spawn(SpriteBundle {
        texture: sprite,
        transform: Transform {
            translation: spawn_position,
            rotation: Quat::from_rotation_z(rotation),
            scale: Vec3::new(1.0, 1.0 ,1.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Name::new(name))
    .insert(MeteorComponent {
        size: size,
        rotation_speed: rotation_speed,
    })
    .insert(HitBoxSize(get_size_from_type(size)))
    .insert(Velocity(Vec2::from(velocity)))
    .insert(Position(Vec2::new(
        position.x,
        position.y,
    )))
    .insert(RotationAngle(rotation))
    .insert(BoundsDespawnableWithTimer::new(bounds_offset, 3.0, 1.0))
    .insert(CollisionDespawnableWithDamage::new(true, get_damage_from_type(size)))
    .insert(CleanUpEndGame::new(true))
    ;
}

fn get_damage_from_type(_size_type: MeteorSizeType) -> f32 {
    for (st, stats) in METEOR_DMG {
        if st == _size_type {
            return stats;
        }
    }
    METEOR_DMG[0].1.clone()
}

fn get_size_from_type(_size_type: MeteorSizeType) -> Vec2 {
    for (st, size) in METEOR_SIZE {
        if st == _size_type {
            return size;
        }
    }
    METEOR_SIZE[0].1.clone()
}