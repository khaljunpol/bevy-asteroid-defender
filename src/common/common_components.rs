use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;

#[derive(Component, InspectorOptions)]
pub struct HitBoxSize(pub Vec2);

#[derive(Component, InspectorOptions)]
pub struct Velocity(pub Vec2);

#[derive(Component, InspectorOptions)]
pub struct Position(pub Vec2);

#[derive(Component, InspectorOptions)]
pub struct RotationAngle(pub f32);

#[derive(Component)]
pub struct BoundsDespawnable(pub Vec2);

#[derive(Component, InspectorOptions)]
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

#[derive(Component, InspectorOptions)]
/**
 * Will Despawn on Collision
 */
pub struct CollisionDespawnableWithDamage {
    pub should_damage: bool,
    pub damage: i32
}

impl CollisionDespawnableWithDamage {
    pub fn new(should_damage: bool, damage: i32) -> Self {
        CollisionDespawnableWithDamage { should_damage, damage }
    }
}

#[derive(Component, InspectorOptions)]
pub struct BoundsWarpable();


#[derive(Component, InspectorOptions)]
/**
 * Spawned component when Meteor is collided
 */
pub struct MeteorCollision {
    pub size: i32,
    pub translation: Vec3
}

#[derive(Component, InspectorOptions)]
/**
 * Spawned component when Meteor is collided
 */
pub struct DamageCollision(pub i32);