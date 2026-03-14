use bevy::prelude::*;

#[derive(Component)]
pub struct HitBoxSize(pub Vec2);

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct RotationAngle(pub f32);

/// Entity wraps around screen edges (used for the player ship).
#[derive(Component)]
pub struct BoundsWarpable;

/// Entity despawns when it crosses the window border.
#[derive(Component)]
pub struct BoundsDespawnable(pub Vec2);

/// Entity despawns on collision with the player and optionally deals damage.
#[derive(Component)]
pub struct CollisionDespawnableWithDamage {
    pub should_damage: bool,
    pub damage:        i32,
}

impl CollisionDespawnableWithDamage {
    pub fn new(should_damage: bool, damage: i32) -> Self {
        CollisionDespawnableWithDamage { should_damage, damage }
    }
}

/// Temporary marker used to carry damage into the collision processing system.
#[derive(Component)]
pub struct DamageCollision(pub i32);

/// Temporary marker emitted when a meteor loses its last HP.
/// Carries the position for spawning split fragments.
#[derive(Component)]
pub struct MeteorSplitEvent {
    pub size:        i32,   // parent size ordinal; children are (size - 1)
    pub translation: Vec3,
}
