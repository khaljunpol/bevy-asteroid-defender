use bevy::prelude::*;

/// Marks an entity to be despawned when the current run ends (StartGame reset).
/// Use this for: the player, anything that must not survive a full restart.
#[derive(Component)]
pub struct CleanUpOnGameOver;

/// Marks an entity to be despawned between levels (on InGame exit or LevelComplete).
/// Use this for: projectiles, powerups.
#[derive(Component)]
pub struct CleanUpOnLevelEnd;

/// Generic cleanup system – despawns all entities with marker component `T`.
pub fn cleanup_system<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
