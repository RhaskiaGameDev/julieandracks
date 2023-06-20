use bevy::ecs::component::Component;
use bevy::prelude::*;
use rand::Rng;
pub(crate) fn spawn_enemies(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();
    commands.spawn(SpriteBundle {
        texture: asset_server.load("fly.png"),
        transform: Transform::from_translation(Vec3::new(rng.gen_range(-240.0..240.0), -130., 0.)),
        ..default()
    });
}
