use bevy::prelude::*;
use lib::{
    Stats, ShipType, DEFAULT_STATS
};
use rand::{
    prelude::*
};

#[derive(Component)]
pub struct ShipComponent {
    pub ship_type: ShipType,
    pub stats: Stats
}

impl ShipComponent {
    pub fn new() -> ShipComponent {
        let ship_type = Self::randomize_type();
        let stats = Self::get_stats_from_type(ship_type);

        ShipComponent { ship_type, stats }
    }

    fn randomize_type() -> ShipType {
        let mut rng = thread_rng();
        // Generate a random ShipType
        rng.gen()
    }

    fn get_stats_from_type(ship_type: ShipType) -> Stats {
        for (st, stats) in DEFAULT_STATS {
            if matches!(st, ship_type) {
                return stats;
            }
        }
        DEFAULT_STATS[0].1.clone()
    }
}

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

fn randomize_type_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut ShipComponent, With<ShipComponent>>)
{
    if let Ok(mut ship) = query.get_single_mut() {
        
    }
}