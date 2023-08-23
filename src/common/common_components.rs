use bevy::prelude::*;
use lib::Stats;

#[derive(Component)]
pub struct HitBoxSize(pub Vec2);

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct RotationAngle(pub f32);

#[derive(Component)]
pub struct StatsComponent(pub Stats);

#[derive(Component)]
pub struct BoundsDespawnable(pub Vec2);

#[derive(Component)]
pub struct BoundsDespawnableWithTimer{
    pub bounds: BoundsDespawnable,
    pub initial_spawn_timer: Timer,
    pub despawn_timer: Timer,
    pub should_despawn: bool
}

impl BoundsDespawnableWithTimer {
    pub fn new(bounds: Vec2, despawn_delay: f32, spawn_check_delay: f32) -> BoundsDespawnableWithTimer {
        let timer = Timer::from_seconds(spawn_check_delay, TimerMode::Once);
        let timer2 = Timer::from_seconds(despawn_delay, TimerMode::Once);

        BoundsDespawnableWithTimer { 
            bounds: BoundsDespawnable(bounds), 
            despawn_timer: timer2, 
            initial_spawn_timer: timer,
            should_despawn: false
        }
    }
}

#[derive(Component)]
pub struct BoundsWarpable();

#[derive(Component)]
pub struct MeteorCollisionComponent {
    pub size: i32,
    pub translation: Vec3
}