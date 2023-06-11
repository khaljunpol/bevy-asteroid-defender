use bevy::prelude::*;
use rand::{
    prelude::*
};

use lib::{METEOR_BIG_SIZE, METEOR_MAX_COUNT};
use crate::{
    common_components::{RotationAngle, Velocity, Position, BoundsDespawnable, HitBoxSize},
    resources::{GameSprites, WindowSize}, 
    utils::{get_angle_to_target, calculate_max_spawn_distance}, 
    player::PlayerComponent
};

#[derive(Component)]
pub struct MeteorComponent{
    pub rotation_speed: f32
}

impl MeteorComponent{
    pub fn new(rotation_speed: f32) -> MeteorComponent {
        MeteorComponent {rotation_speed}
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
    game_sprites: Res<GameSprites>,
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
    let center = Vec2::new(wdw_size.w / 2.0, wdw_size.h / 2.0);
    
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
        let max_dist = calculate_max_spawn_distance(angle, Vec2 { x: wdw_size.w, y: wdw_size.h });

        let (x_pos_rand, y_pos_rand) = angle.to_radians().sin_cos();
    
        let position = Vec2::new(x_pos_rand * max_dist, y_pos_rand * max_dist);
    
        // randomizing the starting rotation angle of the powerups
        let rotation = rng.gen_range(-0.05..0.05) as f32;
    
        // randomizing rotation speed
        let rot_speed =
            rng.gen_range(-0.05..0.05) as f32;

        let rot_velocity = get_angle_to_target(player_position.0, position);

        let powerup_position = Vec3::new(position.x, position.y, 1.0);
    
        commands
            .spawn(SpriteBundle {
                texture: game_sprites.meteor_big.clone(),
                transform: Transform {
                    translation: powerup_position,
                    rotation: Quat::from_rotation_z(rotation),
                    scale: Vec3::new(1.0, 1.0 ,1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Name::new("Meteor"))
            .insert(MeteorComponent::new(rot_speed))
            .insert(HitBoxSize(METEOR_BIG_SIZE))
            .insert(Velocity(Vec2::from(rot_velocity)))
            .insert(Position(Vec2::new(position.x, position.y)))
            .insert(RotationAngle(rotation))
            .insert(BoundsDespawnable(Vec2::new(center.x, center.y)));
        
    }
}
