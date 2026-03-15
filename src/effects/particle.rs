use bevy::prelude::*;
use rand::{thread_rng, Rng};

use lib::MeteorSizeType;
use crate::resources::GameSprites;

#[derive(Component)]
pub struct ParticleComponent {
    pub lifetime:     f32,
    pub max_lifetime: f32,
    pub velocity:     Vec2,
}

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, particle_update_system);
    }
}

fn particle_update_system(
    mut commands: Commands,
    time:         Res<Time>,
    mut query:    Query<(Entity, &mut ParticleComponent, &mut Transform, &mut Sprite)>,
) {
    let dt = time.delta_seconds();
    for (entity, mut particle, mut tf, mut sprite) in &mut query {
        particle.lifetime -= dt;

        tf.translation.x += particle.velocity.x * dt;
        tf.translation.y += particle.velocity.y * dt;

        let alpha = (particle.lifetime / particle.max_lifetime).max(0.0);
        sprite.color = Color::rgba(1.0, 1.0, 1.0, alpha);

        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn spawn_explosion(
    commands:     &mut Commands,
    game_sprites: &GameSprites,
    position:     Vec3,
    size:         MeteorSizeType,
) {
    let mut rng = thread_rng();

    let (count, speed_max, lifetime_min, lifetime_max) = match size {
        MeteorSizeType::Large  => (10usize, 120.0_f32, 0.4_f32, 0.9_f32),
        MeteorSizeType::Medium => (6,        80.0,      0.3,     0.7),
        MeteorSizeType::Small  => (4,        50.0,      0.2,     0.5),
    };

    for _ in 0..count {
        let angle    = rng.gen_range(0.0_f32..std::f32::consts::TAU);
        let speed    = rng.gen_range(20.0_f32..speed_max);
        let vel      = Vec2::new(angle.cos() * speed, angle.sin() * speed);
        let lifetime = rng.gen_range(lifetime_min..lifetime_max);

        let texture: Handle<Image> = match rng.gen_range(0_u8..3) {
            0 => game_sprites.star1.clone(),
            1 => game_sprites.star2.clone(),
            _ => game_sprites.star3.clone(),
        };

        let scale = rng.gen_range(0.15_f32..0.45);

        commands.spawn((
            SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(position.x, position.y, 5.0),
                    scale: Vec3::splat(scale),
                    ..default()
                },
                ..default()
            },
            ParticleComponent {
                lifetime,
                max_lifetime: lifetime,
                velocity: vel,
            },
            Name::new("Explosion Particle"),
        ));
    }
}
