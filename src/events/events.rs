use bevy::prelude::*;

use crate::{
    resources::{CameraShake, Life},
    state::states::GameStates,
    utils::manager::goto_game_over,
};

#[derive(Event)]
pub struct PlayerDeadEvent;

#[derive(Event)]
pub struct PlayerSpawnEvent;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayerDeadEvent>()
            .add_event::<PlayerSpawnEvent>()
            .add_systems(
                Update,
                (
                    check_player_dead,
                    goto_game_over.run_if(on_event::<PlayerDeadEvent>()),
                    on_player_death_shake.run_if(on_event::<PlayerDeadEvent>()),
                )
                    .run_if(in_state(GameStates::InGame)),
            );
    }
}

/// Fires `PlayerDeadEvent` when HP drops to zero.
pub fn check_player_dead(
    life:        Res<Life>,
    mut ev_dead: EventWriter<PlayerDeadEvent>,
) {
    if life.current_life <= 0 {
        ev_dead.send(PlayerDeadEvent);
    }
}

fn on_player_death_shake(mut shake: ResMut<CameraShake>) {
    shake.trigger(15.0);
}
