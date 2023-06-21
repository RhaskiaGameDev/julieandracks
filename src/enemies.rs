use bevy::ecs::component::Component;
use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
struct Enemy {
    row: usize,
    speed: f32,
    //extra if needed
}


pub(crate) fn spawn_enemies(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();
    let row: usize = rng.gen_range(0..4);

    commands.spawn((SpriteBundle {
        texture: asset_server.load("fly.png"),
        transform: Transform::from_translation(Vec3::new((row as f32 - 1.5) * 32, -130., 0.)),
        ..default()
    }, Enemy { row }));
}
