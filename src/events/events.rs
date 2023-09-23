use bevy::{prelude::*, ecs::schedule::ScheduleLabel};

use crate::{state::states::GameStates, player::player::PlayerComponent, common::common_components::Life};

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
pub struct ChangeStateEvent{
    previous: GameStates,
    next: GameStates
}
impl ChangeStateEvent {
    pub fn new(current: GameStates, next: GameStates) -> ChangeStateEvent {
        ChangeStateEvent { previous: current, next }
    }
}

#[derive(Event)]
pub struct StateStartEvent(pub GameStates);
#[derive(Event)]
pub struct StateEndEvent(pub GameStates);

pub struct EventsPlugin;
impl Plugin for EventsPlugin {
    
    fn build(&self, app: &mut App) {
        // Create a schedule to store our systems
        let mut schedule = Schedule::default();
        schedule.add_systems(Events::<StateStartEvent>::update_system.in_set(FlushEvents))
            .add_systems(Events::<StateEndEvent>::update_system.in_set(FlushEvents))
            .add_systems(Events::<ChangeStateEvent>::update_system.in_set(FlushEvents))
            .add_systems(Events::<PlayerDeadEvent>::update_system.in_set(FlushEvents));

        app
        .insert_resource(Events::<StateStartEvent>::default())
        .insert_resource(Events::<StateEndEvent>::default())
        .insert_resource(Events::<ChangeStateEvent>::default())
        .insert_resource(Events::<PlayerDeadEvent>::default())

        .add_schedule(EventSchedule, schedule)

        .add_systems(Update, (
            trigger_player_dead_event.after(FlushEvents),
            trigger_state_start_event.after(FlushEvents),
            trigger_next_state_event.after(FlushEvents),
            trigger_next_state_event.after(FlushEvents),
        ));
    }
}

fn trigger_player_dead_event(
    mut player_query: Query<(&Life), With<PlayerComponent>>,
    mut ev_played_dead: EventWriter<PlayerDeadEvent>,
) {
    for (player_life) in player_query.iter_mut() {
        if player_life.current_life <= 0.0 {
            ev_played_dead.send(PlayerDeadEvent);
        }
    }
}

pub fn trigger_state_start_event(
    app_state: Res<State<GameStates>>,
    mut ev_start_state: EventWriter<StateStartEvent>){
    ev_start_state.send(StateStartEvent(*app_state.get()));
}

pub fn trigger_state_end_event(
    app_state: Res<State<GameStates>>,
    mut ev_start_state: EventWriter<StateEndEvent>){
    ev_start_state.send(StateEndEvent(*app_state.get()));
}

pub fn trigger_next_state_event(
    app_state: Res<State<GameStates>>,
    mut next_state: ResMut<NextState<GameStates>>, 
    mut ev_change_state: EventWriter<ChangeStateEvent>,
){
    println!("{:?}", app_state.get());
    // next_state.set(app_state.get().next());
    ev_change_state.send(ChangeStateEvent::new(*app_state.get(), app_state.get().next()));
}


// fn state_change_event