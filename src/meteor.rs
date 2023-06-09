use bevy::prelude::*;
use rand::{
    prelude::*
};

use lib::{METEOR_BIG_SIZE, METEOR_MAX_COUNT};
use crate::{
    common_components::{RotationAngle, Velocity, Position, BoundsDespawnable, HitBoxSize},
    resources::{GameSprites, WindowSize}
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
    for (powerup, mut rotation_angle) in query.iter_mut() {
        rotation_angle.0 += powerup.rotation_speed;
    }
}

pub fn spawn_meteor_system(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    wdw_size: Res<WindowSize>,
    query: Query<With<MeteorComponent>>,
)
{
    let mut count = 0;
    for _ in query.iter() {
        count += 1;
    }
    
    let center = Vec2::new(wdw_size.w / 2.0, wdw_size.h / 2.0);
    let mut rng = thread_rng();

    if count < METEOR_MAX_COUNT {
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
        let mut x_speed = rng.gen_range(-1.5..1.5);
        let mut y_speed = 0.0;

        if position.y > center.y{
            y_speed = rng.gen_range(-1.5..-1.0);
        } else if position.y < center.y{
            y_speed = rng.gen_range(1.0..1.5);
        }

        let mut speed = Vec2::new(x_speed, y_speed);
    
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
            .insert(Name::new("Power Up"))
            .insert(MeteorComponent::new(rot_speed))
            .insert(HitBoxSize(METEOR_BIG_SIZE))
            .insert(Velocity(Vec2::from(speed)))
            .insert(Position(Vec2::new(position.x, position.y)))
            .insert(RotationAngle(rotation))
            .insert(BoundsDespawnable());
    }
}