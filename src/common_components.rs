use bevy::prelude::*;
use lib::Stats;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct RotationAngle(pub f32);

#[derive(Component)]
pub struct StatsComponent(pub Stats);