use bevy::ecs::component::Component;
use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Enemy {
    pub row: usize,
    pub speed: f32,
    //extra if needed
}


pub(crate) fn spawn_enemies(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();
    let row: usize = rng.gen_range(0..5);

    commands.spawn((SpriteBundle {
        texture: asset_server.load("fly.png"),
        transform: Transform::from_translation(Vec3::new((row as f32 - 2.) * 32., -130., 0.)),
        ..default()
    }, Enemy { row, speed: 10. }));
}
