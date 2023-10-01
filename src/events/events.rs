use bevy::{prelude::*, ecs::schedule::ScheduleLabel};

use crate::{state::states::GameStates, player::player::PlayerComponent, common::common_components::Life, utils::manager::game_end};

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct EventSchedule;

// Events need to be updated in every frame in order to clear our buffers.
// This update should happen before we use the events.
// Here, we use system sets to control the ordering.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FlushEvents;

#[derive(Event)]
pub struct PlayerDeadEvent;

#[derive(Event)]
pub struct PlayerSpawnEvent;

#[derive(Event)]
pub struct StateStartEvent(pub GameStates);
#[derive(Event)]
pub struct StateEndEvent(pub GameStates);

pub struct EventsPlugin;
impl Plugin for EventsPlugin {
    
    fn build(&self, app: &mut App) {
        app
        .init_resource::<Events<StateStartEvent>>()
        .init_resource::<Events<StateEndEvent>>()
        .init_resource::<Events<PlayerDeadEvent>>()
        .init_resource::<Events<PlayerSpawnEvent>>()

        .add_systems(Update, (
                //Handle events after the event is called
                (handle_player_dead_event, game_end).chain()
                    .run_if(on_event::<PlayerDeadEvent>())
                    .before(event_cleanup::<PlayerDeadEvent>),

                handle_state_start_event.run_if(on_event::<StateStartEvent>())
                    .before(event_cleanup::<StateStartEvent>),
                handle_state_end_event.run_if(on_event::<StateEndEvent>())
                    .before(event_cleanup::<StateEndEvent>)
        ));
    }
}

/// Custom cleanup strategy for events
/// Generic to allow using for any custom event type
pub fn event_cleanup<T: Event>(
    mut events: ResMut<Events<T>>,
) {
    // clean up events
    events.clear();
}

pub fn check_player_dead_event(
    mut player_query: Query<(&Life), With<PlayerComponent>>,
    mut ev_played_dead: EventWriter<PlayerDeadEvent>,
) {
    for player_life in player_query.iter_mut() {
        if player_life.current_life <= 0.0 {
            ev_played_dead.send(PlayerDeadEvent);
        }
    }
}

pub fn send_state_start_event(
    app_state: Res<State<GameStates>>,
    mut ev_start_state: EventWriter<StateStartEvent>){
        info!("Sending start state event!");
    ev_start_state.send(StateStartEvent(*app_state.get()));
}

pub fn send_state_end_event(
    app_state: Res<State<GameStates>>,
    mut ev_start_state: EventWriter<StateEndEvent>){
        info!("Sending end state event!");
    ev_start_state.send(StateEndEvent(*app_state.get()));
}

fn handle_player_dead_event(
    next_state: ResMut<NextState<GameStates>> 
){
    // Handle dead event here
}

fn handle_state_start_event(
    mut event_reader: EventReader<StateStartEvent>
){
    for event in event_reader.iter() {
        info!("Trigger start event! {:?}", event.0);
        break
    }
}

fn handle_state_end_event(
    mut event_reader: EventReader<StateEndEvent>
){
    for event in event_reader.iter() {
        info!("Trigger end event! {:?}", event.0);
        break
    }
}