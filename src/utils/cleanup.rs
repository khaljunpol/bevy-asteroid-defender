use bevy::prelude::*;

#[derive(Component)]
pub struct CleanUpEndGame {
    pub despawn_entity: bool
}

impl CleanUpEndGame {
    pub fn new(despawn_entity: bool) -> Self {
        CleanUpEndGame {
            despawn_entity,
        }
    }
}

pub fn cleanup_system<T: Component>(
    mut commands: Commands,
    endgame_query: Query<(Entity, &CleanUpEndGame), With<CleanUpEndGame>>
) {
    
    for (entity, cleanup) in endgame_query.iter() {
        if cleanup.despawn_entity{
            commands.entity(entity).despawn_recursive();
        }
    }
}