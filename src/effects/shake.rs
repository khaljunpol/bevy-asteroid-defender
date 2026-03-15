use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::resources::CameraShake;

pub struct CameraShakePlugin;

impl Plugin for CameraShakePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CameraShake>()
            .add_systems(Update, camera_shake_system);
    }
}

fn camera_shake_system(
    time:      Res<Time>,
    mut shake: ResMut<CameraShake>,
    mut cam_q: Query<&mut Transform, With<Camera>>,
) {
    if shake.intensity <= 0.0 {
        if let Ok(mut tf) = cam_q.get_single_mut() {
            tf.translation.x = 0.0;
            tf.translation.y = 0.0;
        }
        return;
    }

    let mut rng = thread_rng();
    let offset_x = rng.gen_range(-shake.intensity..shake.intensity);
    let offset_y = rng.gen_range(-shake.intensity..shake.intensity);

    if let Ok(mut tf) = cam_q.get_single_mut() {
        tf.translation.x = offset_x;
        tf.translation.y = offset_y;
    }

    // Decay at 25 units/second
    shake.intensity -= 25.0 * time.delta_seconds();
    shake.intensity = shake.intensity.max(0.0);
}
