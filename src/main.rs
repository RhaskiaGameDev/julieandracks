use bevy::prelude::*;

#[derive(Component)]
struct Health
{
    health: i32,
}

struct Plant
{
    name: String,
}

#[derive(Component)]
struct PlantBed
{
    plant: Option<Plant>,
    row: i32,
    column: i32
}

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_beds)
        .run();
}

const ROWS: i32 = 4;
const COLUMNS: i32 = 5;

fn spawn_beds(mut commands: Commands,
              asset_server: Res<AssetServer>)
{
    commands.spawn(Camera2dBundle::default());

    for x in 0..ROWS
    {
        for y in 0..COLUMNS
        {
            commands.spawn((
               SpriteBundle {
               texture: asset_server.load("bed.png"),
               ..default() },
               PlantBed{plant: None, row: x, column: y }));
        }
    }
}
