use bevy::prelude::*;
use bevy_hanabi::prelude::*;

use super::particle::ParticleComponent;

pub fn spawn_particle(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, mut effects: ResMut<Assets<EffectAsset>>)
    {
    // let mut color_gradient1 = Gradient::new();
    // color_gradient1.add_key(0.0, Vec4::new(4.0, 4.0, 4.0, 1.0));
    // color_gradient1.add_key(0.1, Vec4::new(4.0, 4.0, 0.0, 1.0));
    // color_gradient1.add_key(0.9, Vec4::new(4.0, 0.0, 0.0, 1.0));
    // color_gradient1.add_key(1.0, Vec4::new(4.0, 0.0, 0.0, 0.0));

    // let mut size_gradient1 = Gradient::new();
    // size_gradient1.add_key(0.3, Vec2::new(10.0, 10.0));
    // size_gradient1.add_key(1.0, Vec2::splat(0.0));
    
    // let texture_handle: Handle<Image> = asset_server.load("sprites/effects/star3.png");

    // let effect1 = effects.add(
    //     EffectAsset {
    //         name: "portal".to_string(),
    //         capacity: 100,
    //         spawner: Spawner::rate(10.0.into()),
    //         ..Default::default()
    //     }
    //     .init(InitPositionCircleModifier {
    //         center: Vec3::ZERO,
    //         axis: Vec3::Z,
    //         radius: 100.,
    //         dimension: ShapeDimension::Surface,
    //     })
    //     .init(InitLifetimeModifier {
    //         // Give a bit of variation by randomizing the lifetime per particle
    //         lifetime: Value::Uniform((0.6, 1.3)),
    //     })
    //     .update(LinearDragModifier { drag: 2. })
    //     .update(RadialAccelModifier::constant(Vec3::ZERO, -6.0))
    //     .update(TangentAccelModifier::constant(Vec3::ZERO, Vec3::Z, 30.))
    //     .render(ColorOverLifetimeModifier {
    //         gradient: color_gradient1,
    //     })
    //     .render(SizeOverLifetimeModifier {
    //         gradient: size_gradient1,
    //     })
    //     .render(ParticleTextureModifier {
    //         texture: texture_handle,
    //     })
    //     .render(BillboardModifier {})
    //     .render(OrientAlongVelocityModifier),
    // );

    // commands.spawn((
    //     Name::new("shield_particle"),
    //     ParticleEffectBundle {
    //         effect: ParticleEffect::new(effect1),
    //         transform: Transform::IDENTITY,
    //         ..Default::default()
    //     }
    // ))
    // .insert(ParticleComponent);
}