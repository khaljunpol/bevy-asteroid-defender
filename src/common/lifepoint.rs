use bevy::prelude::*;

#[derive(Component)]
pub struct LifePointComponent{
    pub life: f32
}

pub struct LifePointPlugin;

impl Plugin for LifePointPlugin {
    fn build(&self, app: &mut App) {
        
    }
}