use bevy::prelude::*;

use crate::events::events::send_state_start_event;

#[derive(Component)]
pub struct SetUpStartGameState;

pub fn setup_system<T: Component>(
    mut commands: Commands, q: Query<Entity, With<T>>
) {
}