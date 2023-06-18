use bevy::prelude::*;
use rand::{
    prelude::*
};

use lib::{METEOR_BIG_SIZE, METEOR_MAX_COUNT, METEOR_MED_SIZE, METEOR_SML_SIZE};
use crate::{
    common_components::{RotationAngle, Velocity, Position, BoundsDespawnable, HitBoxSize, MeteorCollisionComponent, BoundsDespawnableWithTimer},
    resources::{GameSprites, WindowSize}, 
    utils::{get_angle_to_target, calculate_max_spawn_distance}, 
    player::PlayerComponent
};

#[derive(Component)]
pub struct MeteorComponent{
    pub size: i32,
    pub rotation_speed: f32
}

impl MeteorComponent{
    pub fn new(rotation_speed: f32) -> MeteorComponent {
        MeteorComponent {
            size: 3,
            rotation_speed
        }
    }
}


pub struct MeteorPlugin;

impl Plugin for MeteorPlugin{
    fn build(&self, app: &mut App) {
        app.add_system(meteor_rotation_system);
    }
}

fn meteor_rotation_system(mut query: Query<(&MeteorComponent, &mut RotationAngle)>) {
    for (meteor, mut rotation_angle) in query.iter_mut() {
        rotation_angle.0 += meteor.rotation_speed;
    }
}

pub fn spawn_meteor_system(
    mut commands: Commands,
    mut game_sprites: Res<GameSprites>,
    wdw_size: Res<WindowSize>,
    query: Query<With<MeteorComponent>>,
    player_query: Query<(&Position, With<PlayerComponent>)>,
)
{
    let (player_position, player) = player_query.single();

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
            3,
            powerup_position,
            position,
            rotation,
            rot_speed,
            rot_velocity,
            Vec2::new(max_dist, max_dist)
        );
    }
}


pub fn meteor_collision_spawn_system(
    mut commands: Commands,
    mut game_sprites: Res<GameSprites>,
    wdw_size: Res<WindowSize>,
    query: Query<(Entity, &MeteorCollisionComponent)>,
) {
    let mut rng = thread_rng();

    for (entity, collision) in query.iter() {
        if collision.size > 0 {
            // split the into smaller pieces
            for i in 1..4 {
                // randomizing rotation angle
                let randomized_rotation_angle = rng.gen_range(-1.0..1.0);

                let speed = i as f32 * 0.75;

                // randomizing movement speed
                let speed = Vec2::new(rng.gen_range(-speed..speed), rng.gen_range(-speed..speed));

                // randomizing rotation speed
                let rotation_speed =
                    rng.gen_range(-0.05..0.05);

                spawn_meteor(
                    &mut commands,
                    &mut game_sprites,
                    collision.size.clone(),
                    collision.translation,
                    Vec2::new(collision.translation.x, collision.translation.y),
                    randomized_rotation_angle,
                    rotation_speed,
                    speed,
                    Vec2::new(50.0, 50.0)
                ); 
            }
        }

        // despawn MeteorCollisionComponent entity
        commands.entity(entity).despawn();
    }
}

fn spawn_meteor(
    commands: &mut Commands,
    game_sprites: &mut Res<GameSprites>,
    size: i32,
    spawn_position: Vec3,
    position: Vec2,
    rotation: f32,
    rotation_speed: f32,
    velocity: Vec2,
    bounds_offset: Vec2
) {
    let name = match size {
        3 => "Big Meteor",
        2 => "Med Meteor",
        1 => "Sml Meteor",
        _ => "Big Meteor"
    };
    let sprite = match size {
        3 => game_sprites.meteor_big.clone(),
        2 => game_sprites.meteor_med.clone(),
        1 => game_sprites.meteor_sml.clone(),
        _ => game_sprites.meteor_big.clone()
    };
    
    let hitbox_size = match size {
        3 => METEOR_BIG_SIZE,
        2 => METEOR_MED_SIZE,
        1 => METEOR_SML_SIZE,
        _ => METEOR_BIG_SIZE
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
    .insert(HitBoxSize(hitbox_size))
    .insert(Velocity(Vec2::from(velocity)))
    .insert(Position(Vec2::new(
        position.x,
        position.y,
    )))
    .insert(RotationAngle(rotation))
    .insert(BoundsDespawnableWithTimer::new(bounds_offset, 3.0, 1.0));
}