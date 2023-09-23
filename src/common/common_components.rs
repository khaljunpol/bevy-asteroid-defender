use bevy::prelude::*;

#[derive(Component)]
pub struct HitBoxSize(pub Vec2);

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct RotationAngle(pub f32);

#[derive(Component)]
pub struct Life {
    pub max_life: f32,
    pub current_life: f32
}

impl Life {
    pub fn new(life: f32) -> Self {
        Life { max_life: life, current_life: life }
    }
}

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
    pub fn new(bounds: Vec2, despawn_delay: f32, spawn_check_delay: f32) -> Self {
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
/**
 * Will Despawn on Collision
 */
pub struct CollisionDespawnableWithDamage {
    pub should_damage: bool,
    pub damage: f32
}

impl CollisionDespawnableWithDamage {
    pub fn new(should_damage: bool, damage: f32) -> Self {
        CollisionDespawnableWithDamage { should_damage, damage }
    }
}

#[derive(Component)]
pub struct BoundsWarpable();


#[derive(Component)]
/**
 * Spawned component when Meteor is collided
 */
pub struct MeteorCollision {
    pub size: i32,
    pub translation: Vec3
}

#[derive(Component)]
/**
 * Spawned component when Meteor is collided
 */
pub struct DamageCollision(pub f32);